use std::env;
use image::{DynamicImage, Luma, ImageBuffer, Rgba};
use imageproc::rect::Rect;
use imageproc::contours::find_contours;
use imageproc::geometry::{contour_area, min_area_rect};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::{filter::gaussian_blur_f32, point::Point};
use imageproc::contrast::threshold;
use imageproc::integral_image::{integral_image,sum_image_pixels};

//为结构体或枚举自动实现这三个有用的特性，而无需手动编写相关的代码
#[derive(Debug, Copy, Clone)]
struct Position {  
  x: i32,  
  y: i32,  
  width: u32,  
  height: u32,  
}

//main 函数是程序的入口点
fn main() {
  println!("identify_result: {:?}", recognize_image());
}

//图片处理
fn recognize_image() -> Vec<u32> {
  let mut img: DynamicImage;
  let mut identify_result:Vec<u32>  = vec![];
  let single_selected_position: Position = Position{x: 110, y: 520, width: 1450, height: 140};
  let option_position: Vec<Position> = vec![
    Position{x: 128, y: 23, width: 38, height: 20},
    Position{x: 175, y: 23, width: 38, height: 20},
    Position{x: 225, y: 23, width: 38, height: 20},
    Position{x: 270, y: 23, width: 38, height: 20}
  ];
  // 单选区域
  let single_rects = Rect::at(single_selected_position.x, single_selected_position.y).of_size(single_selected_position.width, single_selected_position.height);
	//第一题的四个选项区域
  let option_rects = vec![  
      Rect::at(option_position[0].x, option_position[0].y).of_size(option_position[0].width, option_position[0].height),  
      Rect::at(option_position[1].x, option_position[1].y).of_size(option_position[1].width, option_position[1].height), 
      Rect::at(option_position[2].x, option_position[2].y).of_size(option_position[2].width, option_position[2].height), 
      Rect::at(option_position[3].x, option_position[3].y).of_size(option_position[3].width, option_position[3].height), 
  ]; 
  // 打印当前工作目录
  let cwd = env::current_dir().unwrap();
  let path = cwd.join("src/images/paper1.jpg");
  println!("Trying to open image at path: {:?}", path);
  
  // 打开并读取图片，match语句结束后，继续执行后面的代码
  match image::open(path) {
    Ok(image) => {
      img = image.clone();
      println!("成功打开图片，Width: {}, Height: {}", img.width(), img.height());        
    },
    Err(e) => {
        // 处理错误，例如打印错误信息或者返回错误  
        println!("Error opening image: {}", e);
        return identify_result;
    }
  }

  // 从原始图像中获取ROI，截取选择题区域
  let mut img_roi_single_region = DynamicImage::crop(&mut img, single_rects.left() as u32, single_rects.top() as u32, single_rects.width(), single_rects.height());
  
  //将一个彩色图像转换为灰度图像，灰度值为8位（即0-255）的整数值
  let gray_image: image::ImageBuffer<Luma<u8>, Vec<u8>> = img_roi_single_region.to_luma8(); 

  // 高斯模糊
  let blurred_img = gaussian_blur_f32(&gray_image, 1.0);

  // 二值化处理
  // 检查像素值*p是否大于阈值（在这里是180）。如果是，新的像素值将被设置为255（白色）；否则，它将被设置为0（黑色）
  //threshold接收一个灰度图像，并基于提供的阈值生成一个新的二值图像。原始图像保持不变
  //在某些情况下，使用固定的全局阈值可能不是最佳选择。自适应阈值方法可以根据图像的局部特性来动态调整阈值
  let mut binary_image = threshold(&blurred_img, 180u8);
  
  // 画轮廓边框
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
        draw_hollow_rect_mut(
          &mut img_roi_single_region, 
          Rect::at(p.x, p.y).of_size(width, height),
          Rgba([0u8, 0u8, 255u8, 255u8]),
        );
    } 
  }

  // 判断是否填涂，计算每个选项区域内的黑色像素数量。如果黑色像素超过区域总像素70%认为该区域已被填涂
  /*
  for (index, position) in option_position.iter().enumerate() {  
    let mut filled_pixels: u32 = 0;
    for y in (position.y)..(position.y + position.height as i32) {  
      for x in (position.x)..(position.x + position.width as i32) {
        let pixel: &Luma<u8> = ImageBuffer::get_pixel(&binary_image, x as u32, y as u32);
        if pixel.0[0] == 0u8 { // 假设0代表黑色，即填涂颜色  
          filled_pixels += 1; 
        }
      }  
    }  
    println!("第{}个选项，黑色像素个数: {},像素总数{}", index, filled_pixels, (position.width * position.height));
    // 设置一个阈值来判断区域是否被填涂，例如区域内超过70%的像素是黑色的，则视为已填涂
    let filled_ratio = filled_pixels as f32 / ((position.width * position.height) as f32);  
    if filled_ratio > 0.7 {  
      identify_result.push(1);
    } else {  
      identify_result.push(0);  
    }
  }
  */

  //积分图像，判断选项区域是否填涂
  let integral_gray:ImageBuffer<Luma<i64>, Vec<i64>> = integral_image(&binary_image);
  //计算图像中所有像素值的和,分析图像的曝光情况、亮度分布等
  for (index, rect) in option_rects.iter().enumerate() {
    let sum_pixels = sum_image_pixels(
      &integral_gray,
      rect.left() as u32,
      rect.top() as u32,
      (rect.right()-1) as u32,
      (rect.bottom()-1) as u32
    )[0];
    let mean_pixel = sum_pixels / (rect.width() * rect.height()) as i64;
    let filled_ratio = 1.0 - mean_pixel as f32 / 255f32;
    println!("第{}个选项，填涂比{}", index, filled_ratio);
    if filled_ratio > 0.7 {  
      identify_result.push(1);
    } else {  
      identify_result.push(0);  
    } 
  }

  //在图像上绘制矩形及其内容 
  for i in 0..option_rects.len() {  
    draw_hollow_rect_mut(  
        &mut img_roi_single_region,   
        option_rects[i],   
        Rgba([255u8, 0u8, 0u8, 127u8]),  
    ); 
    draw_hollow_rect_mut(  
      &mut binary_image,   
      option_rects[i],   
      image::Luma([0]),  
    );
  }

  // 保存处理后的图像，可以是不同格式
  let binary_img_path = cwd.join("tests/images/binary_img.jpg");  
  binary_image.save(binary_img_path).unwrap();
  let rgba_img_path = cwd.join("tests/images/rgba_img.jpg");  
  img_roi_single_region.save(rgba_img_path).unwrap();
  return identify_result;
}
