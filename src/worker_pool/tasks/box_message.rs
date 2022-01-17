p

pub struct BoxMessageRequest {
	type_is:	String,
	message:	String,
	sender:		PrivateIdentity,
	recipient:	Identity,
}

pub struct BoxMessageResponse {
	type_is:	String,
	nonce:		String,
	boxed_msg:	String,
}

pub handle_box_message(box_req: *BoxMessageRequest) -> BoxMessageResponse {
	let (nonce, boxedMessage) := box_message(box_req.message, box_req.sender, box_req.recipient)
	BoxMessageResponse {type_is: "boxMessage", nonce: nonce, boxed_msg: boxedMessage}
}
