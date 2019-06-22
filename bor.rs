use std::fmt;
// adding a comment
// and one more
fn main() {

	#[derive(Debug,Copy,Clone)]
	struct Xval {
		x:	u32
	};
	impl fmt::Display for Xval {
    	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        	write!(f, "({})", self.x)
	    }
	}
	let x_val1 = Xval {
		x:5u32
	};

	let y_val = x_val1;

	let z_val= x_val1;
	println!("val={}",z_val);
}
