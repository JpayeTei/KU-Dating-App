//rust ma euta variable ma if else condition rakhnamildo raixa if else condition lai ni expression ko rup ma use garna mildo raixa
fn main(){
	let time = 20;
	let greeting = if time < 18 {
	  "Good day."
	} else {
	  "Good evening."
	};
	println!("{}", greeting);
}
