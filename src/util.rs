use std::env;
use octocrab::models::TeamId;

pub fn gh_auth() -> octocrab::Octocrab {
  let token = match env::var("GITHUB_TOKEN") {
    Ok(val) => val,
    Err(err) => panic!("Unable to get a usable GitHub token: {:#?}", err),
  };
  match octocrab::OctocrabBuilder::default()
    .user_access_token(token)
    .build()
  {
    Ok(gh) => gh,
    Err(err) => panic!("Error creating GitHub client: {:#?}", err),
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamTreeNode {
  pub id: TeamId,
  pub name: String,
  pub is_private: bool,
  pub parent_id: Option<TeamId>,
}

const GH_TEAMS_PER_PAGE: u8 = 100;

pub fn pagination_limit() -> u8 {
  match env::var("GH_TEAMS_PER_PAGE") {
    Ok(val) => match val.parse::<u8>() {
      Ok(num) => num,
      Err(err) => {
        eprintln!("Unable to parse GH_TEAMS_PER_PAGE; using default value of {}.\n{:#?}", GH_TEAMS_PER_PAGE, err);
        GH_TEAMS_PER_PAGE
      }
    },
    Err(_) => GH_TEAMS_PER_PAGE,
  }
}