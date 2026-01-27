The equation $Q = rho A v + C$
defines the glacial flow rate.

$ 7.32 beta +
  sum_(i=0)^nabla Q_i / 2

  v := vec(x_1, x_2, x_3) $

$ v := vec(x_1, x_2, x_3) $

= Instroduction(h1)
#lorem(10)

== Background(h2)
#lorem(12)

=== 한글 테스트(h3)

#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge
#import "@preview/cetz:0.4.0"

#show math.equation.where(block: false): it => {
  if target() == "html" {
    html.elem("span", attrs: (role: "math"), html.frame(it))
  } else {
    it
  }
}

#show math.equation.where(block: true): it => {
  if target() == "html" {
    html.elem("figure", attrs: (role: "math"), html.frame(it))
  } else {
    it
  }
}

// for styling, use `where` to assign classes for different types of figure
#show figure: it => {
  if target() == "html" { 
    html.elem("figure", attrs: (class: "typst"), html.frame(it))
  } else {
    it
  }
}

diagram
#figure(
  diagram(cell-size: 0.5mm, $
    K edge("d")\
    S edge("d")\
    F
  $)
)
cetz
#figure(
  cetz.canvas({
    import cetz.draw: *
    set-style(
      stroke: 0.4pt,
      mark: (transform-shape: false, fill: black, scale:0.5)
    )

    line((-1, 0), (1, 0), mark: (end: "stealth"))
    content((1.2, 0), text(top-edge: "x-height")[$x$])
    line((0, -1), (0, 1), mark: (end: "stealth"))
    content((-0.2, 1), $y$)
  })
)
end
