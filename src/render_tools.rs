use std::collections::HashMap;
use macroquad::prelude::{load_texture, Texture2D};

pub struct RenderTools {
	textures: HashMap<String, Texture2D>
}
impl RenderTools {
	pub fn init() -> Self {
		Self {
			textures: HashMap::new()
		}
	}
	
	pub async fn get_texture(&mut self, path: String) -> Texture2D {
		if !self.textures.contains_key(&path) {
			let texture = load_texture(&*path).await.unwrap();
			self.textures.insert(path.clone(), texture);
		}
		self.textures.get(&path).unwrap().clone()
	}
	
}