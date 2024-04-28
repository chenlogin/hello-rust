
#[cfg(test)]
mod tests {
  pub trait HasCoordinates<T> {  
    fn get_coordinates(&self) -> (&T, &T);  
  }  
  
  pub struct Point<T> {  
      pub x: T,  
      pub y: T,  
  }  
    
  impl<T> HasCoordinates<T> for Point<T> {  
      fn get_coordinates(&self) -> (&T, &T) {  
          (&self.x, &self.y)  
      }  
  }
    
  #[test]
  fn it_works() {
    // 创建一个 Point 实例  
    let point = Point { x: 5.0, y: 10.0 };  
      
    // 调用 get_coordinates 方法  
    let (x, y) = point.get_coordinates();  
      
    // 打印出坐标  
    println!("X coordinate: {}", x);  
    println!("Y coordinate: {}", y); 
    assert_eq!(2 + 2, 4);  
  }
}