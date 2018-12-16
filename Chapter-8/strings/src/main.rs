extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
	// for c in "नमस्ते".graphemes(true) {
	// 	println!("{}", &c);
	// }
    for c in UnicodeSegmentation::graphemes("नमस्ते", true) {
        println!("{}", &c);
    }
}
