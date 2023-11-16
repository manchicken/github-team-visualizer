use crate::{
  chunk::util::{gh_auth, pagination_limit, TeamTreeNode},
  CmdArgs,
};
use octocrab::models::{teams::TeamPrivacy, TeamId};
use petgraph::{graph::NodeIndex, visit::Dfs, Graph};
use std::collections::HashMap;

pub async fn print_team_tree(args: &CmdArgs) {
  let gh = gh_auth();
  let mut tree: Graph<TeamTreeNode, u8, petgraph::Directed> = Graph::new();
  let mut flat_index: HashMap<TeamId, NodeIndex> = HashMap::new();
  let mut page_number: u32 = 1;

  let root_node = tree.add_node(TeamTreeNode {
    id: TeamId(0),
    name: String::from("Root (Not a team)"),
    is_private: true,
    parent_id: None,
  });

  // Start by filling up the tree
  'team_pager: loop {
    let teams_req = gh
      .teams(args.organization.as_str())
      .list()
      // It might be helpful to make this respond to an env var?
      .per_page(pagination_limit())
      .page(page_number)
      .send()
      .await;
    if let Err(err) = teams_req {
      panic!("Failed to fetch teams!: {:#?}", err)
    }

    let teams = teams_req.unwrap();
    let has_next = teams.next;
    teams.items.into_iter().for_each(|item| {
      let id = item.id.unwrap();
      let name = item.name.to_owned();
      let is_private = matches!(item.privacy, TeamPrivacy::Open);

      let parent_id = match item.parent {
        Some(p) => Some(p.id),
        _ => None,
      };

      let node = tree.add_node(TeamTreeNode {
        id,
        name,
        is_private,
        parent_id,
      });
      flat_index.insert(id, node);
    });

    if has_next.is_none() {
      break 'team_pager;
    }
    page_number += 1;
  }

  // We're going to iterate over the nodes and reference the flat_index HashMap
  // to more quickly associate the parents.
  tree.node_indices().for_each(|idx| {
    // Skip the root.
    if idx == root_node {
      return;
    }

    // Let's make edges!
    let item = &tree[idx];

    if let Some(parent_id) = item.parent_id {
      let parent_ref = flat_index.get(&parent_id);
      if parent_ref.is_none() {
        panic!(
          "I couldn't find the parent team for {}! Freaking out!",
          item.name
        );
      }

      // Add the edges parent-first
      tree.add_edge(*(parent_ref.unwrap()), idx, 1);
    } else {
      tree.add_edge(root_node, idx, 0);
    }
  });

  print_nested_tree(&tree, root_node);
}

fn print_nested_tree(tree: &Graph<TeamTreeNode, u8, petgraph::Directed>, node: NodeIndex) {
  let mut current_node = node;
  // let mut parents: Vec<&TeamTreeNode> = vec![&tree[current_node]];
  let mut stack_of_parents: Vec<TeamId> = vec![];

  let mut dfs = Dfs::new(tree, current_node);
  'print_loop: while let Some(next_node) = dfs.next(tree) {
    let next_node_ref = &tree[next_node];
    let prev_node_ref = &tree[current_node];

    if next_node_ref.id == TeamId(0) {
      continue 'print_loop;
    }

    if next_node_ref.parent_id.is_none() {
      stack_of_parents.pop();
    } else if prev_node_ref.id == next_node_ref.parent_id.unwrap() {
      stack_of_parents.push(prev_node_ref.id);
    } else if stack_of_parents.last().unwrap() != &next_node_ref.parent_id.unwrap() {
      stack_of_parents.pop();
    }

    println!(
      "{}- {}",
      "  ".repeat(stack_of_parents.len()),
      next_node_ref.name
    );

    current_node = next_node;
  }
}
