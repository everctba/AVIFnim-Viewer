// Landing do AVIFnim Viewer — links pendentes de assets/oficiais do owner (issue avifnim-xhk).
// Quando o link oficial do AVIFnim for definido, preencher AVIFNIM_URL.
const AVIFNIM_URL = ""; // ex.: "https://avifnim.com/?utm_source=avifnim-viewer&utm_medium=landing"

const cta = document.getElementById("cta-avifnim");
cta.addEventListener("click", (event) => {
  if (!AVIFNIM_URL) {
    event.preventDefault();
    alert("O link oficial do AVIFnim será adicionado em breve.");
    return;
  }
  cta.href = AVIFNIM_URL;
});

// Marca os links de download que ainda não têm release publicado.
document.querySelectorAll('a[href*="releases/latest"]').forEach((link) => {
  link.addEventListener("click", () => {
    // O GitHub exibe "releases" vazios como 404; mantemos o link para quando houver release.
  });
});
