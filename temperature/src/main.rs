//Convert temperatures between Fahrenheit and Celsius.

 // they are formual for converting which is use ; 

// Celsius to Fahrenheit: °F = (°C × 9/5) + 32 (or °C × 1.8 + 32)
// Fahrenheit to Celsius: °C = (°F - 32) × 5/9 (or (°F - 32) ÷ 1.8)  

use std::io; 


fn main(){
	
 println!("Enter The Temperaute In Celsius");
	
 let mut temp_celsius = String::new(); 
	
	io::stdin()
	  .read_line(&mut temp_celsius)
	  .expect("Failed to read line");

	  let temp_celsius : f32 = temp_celsius.trim().parse().expect("Please type auh number"); 
		
 		println!("Temperature in Celsius: {}°C", temp_celsius);

	let fahrenheit : f32 = (temp_celsius * 1.8) + 32.0_f32; 

	println!("Temperature in Fahrenheit: {}°F", fahrenheit);
} 
