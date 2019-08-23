fn main(){
	//enum Option<T> {
    //	Some(T),
    //	None,
    //}
	let null_var: Option<i32> = None;
	let not_null: Option<i32> = Some(3);
	fn print_value(v: Option<i32>){
		if let Option::Some(n) = v{
			println!("v = {}", n);
		}
		else{
			println!("No value");
		}
	}
	print_value(null_var);
	print_value(not_null);
}