export const FONT_CHOICES = Object.freeze([
  {
    id: "libertinus",
    label: "Libertinus Serif",
    fontFamily: "Libertinus Serif",
    fontUrl: "https://cdn.jsdelivr.net/gh/typst/typst-assets@v0.13.1/files/fonts/LibertinusSerif-Regular.otf",
  },
  {
    id: "literata",
    label: "Literata",
    fontFamily: "Literata",
    fontUrl: "https://fonts.gstatic.com/s/literata/v40/or3PQ6P12-iJxAIgLa78DkrbXsDgk0oVDaDPYLanFLHpPf2TbPa4F_Y.ttf",
  },
  {
    id: "source-serif-4",
    label: "Source Serif 4",
    fontFamily: "Source Serif 4",
    fontUrl: "https://fonts.gstatic.com/s/sourceserif4/v14/vEFy2_tTDB4M7-auWDN0ahZJW3IX2ih5nk3AucvUHf6OAVIJmeUDygwjivBtrhw.ttf",
  },
  {
    id: "atkinson-hyperlegible",
    label: "Atkinson Hyperlegible",
    fontFamily: "Atkinson Hyperlegible",
    fontUrl: "https://fonts.gstatic.com/s/atkinsonhyperlegible/v12/9Bt73C1KxNDXMspQ1lPyU89-1h6ONRlW45G8WbcNcw.ttf",
  },
  {
    id: "space-grotesk",
    label: "Space Grotesk",
    fontFamily: "Space Grotesk",
    fontUrl: "https://fonts.gstatic.com/s/spacegrotesk/v22/V8mQoQDjQSkFtoMM3T6r8E7mF71Q-gOoraIAEj4PVksj.ttf",
  },
  {
    id: "dm-mono",
    label: "DM Mono",
    fontFamily: "DM Mono",
    fontUrl: "https://fonts.gstatic.com/s/dmmono/v16/aFTR7PB1QTsUX8KYvumzIYQ.ttf",
  },
]);

export function fontFor(id) {
  const font = FONT_CHOICES.find((candidate) => candidate.id === id);
  if (!font) throw new Error(`Unknown font: ${id}`);
  return font;
}
