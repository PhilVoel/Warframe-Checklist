const changes = {};

async function getModData() {
	const allMods = await fetch("/mods").then(res => res.json())
	const ownedMods = await fetch(window.location.pathname + "/data").then(res => res.json())
	const ownedMap = new Map(ownedMods);
	allMods.sort((a,b) => a.name.localeCompare(b.name));
	for (const mod of allMods) {
		const ranks = ownedMap.get(mod.name) || [];

		let container = document.createElement("div");
		container.dataset.modName = mod.name;

		const image = document.createElement("img");
		image.src = "https://wiki.warframe.com/images/" + mod.image;
		container.appendChild(image);

		const link = document.createElement("a");
		link.innerText = mod.name;
		link.href = "https://wiki.warframe.com/w/" + mod.link;
		container.appendChild(link);

		const ranksContainer = document.createElement("div");
		for (let i = 0; i <= mod.max_rank; i++) {
			const cb = document.createElement("input");
			cb.type = "checkbox";
			cb.value = i;
			cb.checked = ranks.includes(i);

			cb.addEventListener("change", () => changes[`${mod.name}|${i}`] = cb.checked);

			const label = document.createElement("label");
			label.appendChild(cb);
			label.appendChild(document.createTextNode(i));

			ranksContainer.appendChild(label);
		}
		container.appendChild(ranksContainer);

		document.getElementById("list").appendChild(container);
	}
}
getModData();

function save() {
	const added = [];
	const removed = [];

	for (const key in changes) {
		const [modName, rank] = key.split("|");
		if (changes[key]) {
			added.push([modName, parseInt(rank)]);
		}
		else {
			removed.push([modName, parseInt(rank)]);
		}
	}

	fetch(window.location.pathname + "/data", {
		method: "PUT",
		body: JSON.stringify(added)
	})
	.then(_ => changes = []);
	fetch(window.location.pathname + "/data", {
		method: "DELETE",
		body: JSON.stringify(removed)
	})
	.then(_ => changes = []);
}

function fuzzyMatch(query, target) {
	if (query.length == 0) return true;

	query = query.toLowerCase();
	target = target.toLowerCase();

	let qi = 0;
	for (let ti = 0; ti < target.length; ti++) {
		if (target[ti] === query[qi]) {
			qi++;
			if (qi === query.length) return true;
		}
	}
	return false;
}

document.getElementById("filter").addEventListener("input", e => {
	const q = e.target.value;
	const items = document.querySelectorAll("#list > div");

	for (const el of items) {
		const name = el.dataset.modName;
		if (fuzzyMatch(q, name)) el.style.display = "";
		else el.style.display = "none";
	}
});
