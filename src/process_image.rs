use std::env;

use image::{DynamicImage, GenericImage, Rgba, Luma};  
use imageproc::morphology::dilate_mut; 
use imageproc::distance_transform::Norm;
use imageproc::contours::find_contours;
use imageproc::geometry::{contour_area, min_area_rect};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use imageproc::point::Point;

//图片处理
fn process_image() {
  // 打印当前工作目录  
  let cwd = env::current_dir().unwrap();  
  println!("Current working directory: {:?}", cwd); 

  let path = cwd.join("src/images/paper1.jpg");  
  println!("Trying to open image at path: {:?}", path);

  // 打开并读取图片
  // .unwrap()用于处理 Result 类型。Result 是一个枚举，它可以包含两种状态：Ok(T) 和 Err(E)。
  //如果 Result 是 Ok(T)，它会返回 T。但是，如果 Result 是 Err(E)，.unwrap() 会导致程序异常终止
  //返回一个表示图像数据的枚举的实例。DynamicImage 是一个包含图像数据和相关信息的结构
  //let mut img: DynamicImage = image::open(path).unwrap();
  //Option<T> 是一个枚举类型，用于表示某个值可能存在（Some(T)）或不存在（None）
  // let mut img: Option<DynamicImage> = None;
  let mut img: DynamicImage = DynamicImage::new_rgb8(10, 10);
  match image::open(path) {
      Ok(image) => {
          println!("成功打开图片");  
          img = image;
      },
      Err(e) => {
          // 处理错误，例如打印错误信息或者返回错误  
          eprintln!("Error opening image: {}", e);
      }
  }

  //match语句结束后，继续执行后面的代码
  println!("Width: {}", img.width());  
  println!("Height: {}", img.height());
  img.put_pixel(10, 10, Rgba([255, 0, 0, 1])); // 在坐标(10, 10)处放置一个红色像素

  //将一个彩色图像转换为灰度图像
  //img.to_luma8()返回灰度图像的LumaImage对象，将原始的彩色图像转换为灰度图像，并且灰度值表示为8位（即0-255）的整数值
  //可以直接使用 image 库的 to_luma8 方法转换图像到灰度图像，而不需要 imageproc 的额外转换。to_luma8 方法将图像转换为灰度，并且结果已经是 Luma 类型的 ImageBuffer，这通常可以直接用于后续的图像处理
  //convert_to_luma接受一个LumaImage对象，并返回一个Luma类型的ImageBuffer。虽然to_luma8()已经给了一个灰度图像，但convert_to_luma可能是为了确保或转换图像到一个与imageproc库更兼容的格式。
  //let mut gray_image = imageproc::convert_to_luma(img.to_luma8());
  let mut gray_image: image::ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
  
  // 二值化处理
  // 检查像素值*p是否大于阈值（在这里是128）。如果是，新的像素值将被设置为255（白色）；否则，它将被设置为0（黑色）
  let threshold = 128; // 根据实际情况调整阈值
  let mut binary_image = gray_image.clone();
  for pixel in binary_image.pixels_mut() {  
      // `pixel` 是一个可变引用，可以直接修改它的值  
      if pixel[0] > threshold {
          *pixel = Luma([255]); // 设置像素为白色
      } else {
          *pixel = Luma([0]); // 设置像素为黑色  
      } 
  }

  // 形态学膨胀操作，连接断裂的轮廓，直接修改传入的图像
  // &mut表示一个可变引用，它允许函数修改引用的值，L1范数通常指的是曼哈顿距离（Manhattan distance）
  // 对binary_image执行一次基于L1范数（或曼哈顿距离）的膨胀操作
  dilate_mut(&mut binary_image, Norm::L1, 1);  

  // 查找轮廓并绘制矩形框
  let contours = find_contours(&binary_image);
  for contour in contours {
    // 可以根据轮廓的大小、形状等特征进行筛选，以确定是否为填涂区域
    // 这里简单地将面积大于某个阈值的轮廓视为填涂区域
    let area = contour_area(&contour.points);
    if area > 100.0 { // 阈值100可以根据实际情况调整

        let bounding_rect = min_area_rect(&contour.points);
        let p = Point::new(bounding_rect[0].x, bounding_rect[0].y);
        let width = (bounding_rect[1].x - bounding_rect[0].x) as u32;
        let height = (bounding_rect[3].y - bounding_rect[0].y) as u32;
        //在图像上绘制矩形及其内容 
        draw_filled_rect_mut(
          &mut gray_image, 
          Rect::at(p.x, p.y).of_size(width, height), 
          Luma([0]),
        );
    } 
  }  

  let processed_path = cwd.join("src/images/processed_paper.jpg");  
  // 保存处理后的图像，可以是不同格式
  gray_image.save(processed_path).unwrap();
}
