import { template } from "../../declarations/template";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with template actor, calling the greet method
  const greeting = await template.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
