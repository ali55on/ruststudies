use oop_blog::{Post, NewPost};

fn main() {

    // Modo OOp
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    //---------------------------------
    // Modo Struct (Transição de tipos)
    let mut post = NewPost::new();
    
    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();
    let post = post.approve();
    
    assert_eq!("I ate a salad for lunch today", post.content());
}
