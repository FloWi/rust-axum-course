#[derive(Clone, Debug)]
pub struct Ctx {
	user_id: u64,
}

// using these accessors we make sure that no one outside this module can change the  user_id.
// Constructor
impl Ctx {
	pub fn new(user_id: u64) -> Self {
		Self { user_id }
	}
}

//Property Accessors
impl Ctx {
	pub fn user_id(&self) -> u64 {
		self.user_id
	}
}
