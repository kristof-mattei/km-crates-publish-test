use km_crates_publish_test::reverse;
use pretty_assertions::assert_eq;

fn main() {
    let s = String::from("test");

    let reversed = reverse(s);

    assert_eq!(reversed, "tset");
}
