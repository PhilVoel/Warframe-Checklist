document.getElementById("open_form").addEventListener("submit", e => {
	e.preventDefault();
	const id = document.getElementById("open_list_id").value;
	document.location = "/list/" + id;
});
