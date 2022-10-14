use std::path::{PathBuf, Path};
use image::{io::Reader as ImageReader, DynamicImage, imageops::FilterType};

pub struct PostImage {
  pub name: String,
  image: DynamicImage,
  small_image: DynamicImage,
}

impl PostImage {
  pub fn save(&self) {
    let image_name = format!("{}.webp", self.name.clone());
    let small_image_name = format!("{}.small.webp", self.name.clone());
    let path = Path::new("out").join("images");
    self.image.save(path.join(image_name)).unwrap();
    self.small_image.save(path.join(small_image_name)).unwrap();
  }
}

impl TryFrom<PathBuf> for PostImage {
  type Error = &'static str;

  fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
    let path = Path::new("images").join(value.clone());
    if let Ok(image_file) = ImageReader::open(path) {
      if let Ok(decoded) = image_file.decode() {
        Ok(Self {
          name: value.with_extension("").file_name().unwrap().to_string_lossy().to_string(),
          small_image: decoded.resize(360, 360, FilterType::CatmullRom),
          image: decoded.resize(1080, 1080, FilterType::CatmullRom),
        })
      } else {
        Err("Failed to decode image")
      }
    } else {
      Err("Failed to find image file")
    }
  }
}
