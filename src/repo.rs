extern crate git2;
use git2::Repository;
use git2::RepositoryState;

pub trait ToString {
    fn to_string(&self) -> String;
}

pub fn state() -> RepositoryState {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    //println!("{} state={:?}", repo.path().display(), repo.state());
    //println!("state={:?}", repo.state());
	if repo.state() == RepositoryState::Clean {
		//println!("clean {:?}", repo.state());
	}
	return repo.state()
}