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

## 2022-06-04 Tiling of Towers

Today I debugged the integer tiling code, and combined it with the tower
extrusion code to make a bunch of towers that fit together in a tiling.

![Example output](figures/test-tiling-output.png)

Next Steps:

* Instead of saving many disjoint models, generate one GLB file with multiple
    primitives
* Also use the glTF extension EXT_mesh_gpu_instancing to make many copies of
    the fundamental domain
* Color each tower a different color! or otherwise adjust material properties
* Design more tilings!
* Clean up the extrusion code so it doesn't create extraneous faces

## 2022-06-09 WIP on GLB Output

The next step is creating GLB output. This is a bit involved given all the
levels of indirection, but so far I think the architecture is starting to
fall in place.

I add a primitive, which in turn adds accessors for each attribute/indices,
which in turn add buffer views. The geometry data is reformatted as `Vec<u8>`
and moved into one big buffer, but the indices are saved in structs for
generating the JSON.

I still have a couple portions of this code to go: triangulating the mesh
to produce the positions/normals/indices, and writing the GLB file. And of
course the inevitable debugging of the GLB output, binary formats are difficult
to get right on the first try

Next Steps:

* Finish GLB output as described above
* I designed some tilings on paper, try them out
* Clean up extrusion code