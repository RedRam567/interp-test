use git_tag::git_tag;

#[test]
fn works() {
    let tag = git_tag!();
    // if it got here it didnt fail
    assert!(!tag.is_empty());
    // let is_a_git_tag = tag.contains("-modified") || tag.l
    // assert!(tag.contains("-m"))
}

#[test]
fn works_extra_args() {
    let tag = git_tag!("--dirty=-dirty");
    // if it got here it didnt fail
    assert!(!tag.is_empty());
    // let is_a_git_tag = tag.contains("-modified") || tag.l
    // assert!(tag.contains("-m"))
}