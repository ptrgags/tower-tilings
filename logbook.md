# Integer Tilings

## 2022-06-02 Prototyping tower generation

I actually haven't started making integer tilings yet,
I wanted to prototype a tile extrusion idea I had.
Basicallye each type of tile will be extruded into towers,
each with a potentially different "profille". A profile
is kinda like a path for generating a solid of revolution,
though in this case it's only applied at the vertices of
an n-gon where n will be between 3 and 12 in most cases.

Next Steps:

* Make the face insertion methods more robust, right now
the mesh is leaving behind old faces that are no longer used.
* Make a Vec3 class instead of using tuples
* Generate a whole tiling of towers!
* Make glTF output (which will require triangulation)

## 2022-06-03 Prototyping Integer Tiling implementation

Today I worked on implementing the integer tiling from
["An integer representation for periodic tilings of the plane by regular polygons"](https://reality.cs.ucl.ac.uk/projects/tilings/sotosanchez2021integer.pdf)
by Soto Sánchez et al. I generate faces from the representation as a `Mesh`
that will eventually be turned into tilings. However, right now it's only
producing some of the faces correctly, so I have some debugging to do in my
near future

Next Steps:

* Debug the mesh generation