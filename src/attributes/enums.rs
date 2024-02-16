#[allow(dead_code)]
pub enum ComponentStatus{
	Info,
	Success,
	Neutral, // Default
	Warning,
	Error
}

#[allow(dead_code)]
pub enum Position{
	TopLeft,
	TopMiddle, // Default
	TopRight,
	Middle,
	BottomLeft,
	BottomMiddle,
	BottomRight,
}