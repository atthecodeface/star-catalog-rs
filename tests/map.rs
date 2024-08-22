use std::error::Error;

#[cfg(feature = "image")]
use geo_nd::{Quaternion, Vector};

use star_catalog::{hipparcos, Catalog};

#[cfg(feature = "image")]
use star_catalog::{Quat, Vec3};

#[cfg(feature = "image")]
type Vec2 = geo_nd::FArray<f64, 2>;

#[cfg(test)]
const EXTRA_ALIASES: &[(usize, &'static str)] = &[
    (61281, "Kappa Draconis"),
    (56211, "Lambda Draconis"),
    (58001, "Phecda"),
    (57111, "3 Draconis"),
    (61384, "6 Draconis"),
    (63076, "8 Draconis"),
    (63432, "9 Draconis"),
    (62423, "7 Draconis"),
    (62512, "HD111456"),
    (62046, "HD110678"),
    (60998, "4 Draconis"),
    (63503, "78 Ursa Majoris A"),
    (60212, "70 Ursa Majoris A"),
    (60978, "74 Ursa Majoris A"),
    (60992, "75 Ursa Majoris A"),
    (58989, "HD105043"),
];

#[cfg(feature = "image")]
const IMG_4917_DATA: &[(&str, usize, usize)] = &[
    ("Dubhe", 3406, 3006),
    ("Megrez", 1318, 2516),
    ("Alioth", 451, 1771),
    ("Phecda", 1265, 3448),
    ("Kappa Draconis", 3096, 493),
    ("Lambda Draconis", 3825, 1364),
    ("3 Draconis", 3315, 1638),
    ("4 Draconis", 3038, 616),
    ("6 Draconis", 3123, 444),
    ("8 Draconis", 2106, 655),
    ("9 Draconis", 2260, 447),
    ("7 Draconis", 2432, 649),
    ("78 Ursa Majoris A", 420, 1555),
    ("HD111456", 1303, 1380),
    ("HD110678", 1527, 1398),
    ("70 Ursa Majoris A", 1358, 2282),
    ("74 Ursa Majoris A", 1278, 2017),
    ("75 Ursa Majoris A", 1336, 1965),
    ("HD105043", 2401, 1858),
];

#[test]
fn test_read_hipparcos_json() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    catalog.add_names(hipparcos::HIP_ALIASES, false)?;
    catalog.add_names(EXTRA_ALIASES, false)?;
    Ok(())
}

#[cfg(feature = "image")]
struct Camera {
    width: usize,
    height: usize,
    cx: usize,
    cy: usize,
    mm_per_px_x: f64,
    mm_per_px_y: f64,
    focal_length: f64,
}

// fn degrees(a: f64) -> f64 {
//    a * 180.0 / std::f64::consts::PI
// }

#[cfg(feature = "image")]
impl Camera {
    fn new() -> Self {
        let width = 5184;
        let height = 3456;
        let cx = width / 2;
        let cy = height / 2;
        let mm_per_px_x = 22.3 / width as f64;
        let mm_per_px_y = 14.9 / height as f64;
        let focal_length = 50.0 * 1.038; //  * 1.03;
        Self {
            width,
            height,
            cx,
            cy,
            mm_per_px_x,
            mm_per_px_y,
            focal_length,
        }
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn within_frame(&self, v: Vec2) -> Option<(usize, usize)> {
        if v[0] < 0. || v[0] >= self.width as f64 {
            return None;
        }
        if v[1] < 0. || v[1] >= self.height as f64 {
            return None;
        }
        Some((v[0] as usize, v[1] as usize))
    }

    fn pxy_of_vec(&self, v: &Vec3) -> Vec2 {
        let tx = v[0];
        let ty = v[1];
        let x = self.cx as f64 + (tx * self.focal_length / self.mm_per_px_x);
        let y = self.cy as f64 - (ty * self.focal_length / self.mm_per_px_y);
        [x, y].into()
    }

    fn vec_of_pxy(&self, xy: &Vec2) -> Vec3 {
        //  Get x and y as offsets in mm on frame
        let x = (xy[0] - (self.cx as f64)) * self.mm_per_px_x;
        let y = (self.cy as f64 - xy[1]) * self.mm_per_px_y;
        //     Lens is at focal length, so tan(angle) is x / f or y/f
        let tx = x / self.focal_length;
        let ty = y / self.focal_length;
        //  Unit vector (from lens to frame, really) is then (tx, ty, 1) normalized assuming linear mapping for lens
        let l2 = 1.0 + tx * tx + ty * ty;
        let l = l2.sqrt();
        [tx / l, ty / l, -1.0 / l].into()
    }

    fn quat_mapping_vector_pairs(f0: &Vec3, f1: &Vec3, t0: &Vec3, t1: &Vec3) -> Quat {
        let f0 = f0.normalize();
        let f1 = f1.normalize();
        let t0 = t0.normalize();
        let t1 = t1.normalize();
        let z_axis = [0., 0., 1.].into();
        let f0_to_z = Quat::rotation_of_vec_to_vec(&f0, &z_axis);
        let t0_to_z = Quat::rotation_of_vec_to_vec(&t0, &z_axis);
        let f1_mapped = f0_to_z.apply3(&f1);
        let t1_mapped = t0_to_z.apply3(&t1);
        let angle_f = f1_mapped[1].atan2(f1_mapped[0]);
        let angle_t = t1_mapped[1].atan2(t1_mapped[0]);
        let angle_rot = angle_t - angle_f;
        let rot_in_z = Quat::of_axis_angle(&z_axis, angle_rot);
        t0_to_z.conjugate() * rot_in_z * f0_to_z
    }
    fn quats_of_data(
        &self,
        catalog: &Catalog,
        comp: &str,
        data: &std::collections::HashMap<String, (usize, usize)>,
    ) -> Vec<Quat> {
        // comp = "HD105043"
        // # comp = "Kappa Draconis"
        let mut quats = vec![];
        let star_comp = catalog.find_name(comp).unwrap();
        let star_comp = &catalog[star_comp];
        let (x, y) = &data[comp];
        let vector_comp = self.vec_of_pxy(&[*x as f64, *y as f64].into());
        for (name, pxy) in data {
            let vector_name = self.vec_of_pxy(&[pxy.0 as f64, pxy.1 as f64].into());
            if *name != comp {
                let star_name = catalog.find_name(name).unwrap();
                let star_name = &catalog[star_name];
                // eprintln!("{name} {vector_comp} {vector_name} {star_name:?}");
                let m = Self::quat_mapping_vector_pairs(
                    &star_comp.vector,
                    &star_name.vector,
                    &vector_comp,
                    &vector_name,
                );
                println!("  Quat: {}", m);
                quats.push(m);
            }
        }
        quats
    }
}

#[cfg(feature = "image")]
#[test]
fn test_quats() -> Result<(), Box<dyn Error>> {
    // let comp = "HD105043";
    let comp = "Kappa Draconis";

    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    catalog.add_names(hipparcos::HIP_ALIASES, false)?;
    catalog.add_names(EXTRA_ALIASES, false)?;
    catalog.derive_data();
    let mut x: std::collections::HashMap<String, (usize, usize)> = std::collections::HashMap::new();
    for (a, b, c) in IMG_4917_DATA {
        x.insert(a.to_string(), (*b, *c));
    }
    let camera = Camera::new();
    let quats = camera.quats_of_data(&catalog, comp, &x);
    let avg = Quat::weighted_average_many(quats.iter().map(|x| (1.0, *x)));
    dbg!(&avg);
    for (name, pxy) in &x {
        let star_name = catalog.find_name(name).unwrap();
        let star_name = &catalog[star_name];
        // let v = camera.vec_of_pxy(&[pxy.0 as f64, pxy.1 as f64].into());
        let v = avg.apply3(&star_name.vector);
        let xy = camera.pxy_of_vec(&v);
        eprintln!("{name} {xy} {pxy:?}");
        // dx = data[name][0] - x
        // dy = data[name][1] - y
        // err2 = dx*dx+dy*dy
        // err = math.sqrt(err2)
        // print(name, err, x, y, data[name])
    }

    let star_comp = catalog.find_name(comp).unwrap();
    let star_comp = &catalog[star_comp];
    let subcube = star_comp.subcube;
    let subcubes = subcube.iter_range(3);
    let subcubes = subcubes.filter(|s| s.may_be_on_sphere());
    let star_iter = catalog.iter_within_subcubes(subcubes);

    use image::GenericImage;
    let mut image = image::DynamicImage::new_rgb8(camera.width() as u32, camera.height() as u32);
    for s in star_iter {
        if !s.brighter_than(7.0) {
            continue;
        }
        let v = avg.apply3(&s.vector);
        if let Some(xy) = camera.within_frame(camera.pxy_of_vec(&v)) {
            // eprintln!("{xy:?}");
            if xy.0 < 8 || xy.0 + 8 >= camera.width() {
                continue;
            }
            if xy.1 < 8 || xy.1 + 8 >= camera.height() {
                continue;
            }
            for dx in 0..17 {
                image.put_pixel(xy.0 as u32 + dx - 8, xy.1 as u32, [128, 255, 255, 0].into());
            }
            for dy in 0..17 {
                image.put_pixel(xy.0 as u32, xy.1 as u32 + dy - 8, [128, 255, 255, 0].into());
            }
        }
    }
    image.save("test.png")?;
    // assert!(false);
    Ok(())
}

// vectors = {}
// print(22.3/5184, 14.9/3456)
// for (name, (x,y)) in data.items():
//     vectors[name] = vec_of_pxy(x,y)
//     pass

/*
extra_data = [
    (1813, 1236),
    (1746, 1117),
    (3375, 686),
#     (1300, 1376),
    (420,1556),
    ]

# What star is at 2400 1859?

comp = "HD105043"
# comp = "Kappa Draconis"
quats = []
for name in data:
    if name != comp:
        print( name, data[name], vectors[name], stars[name])
        print( "  Angles", math.degrees(stars[comp].angle_to(stars[name])),  math.degrees(angle_between(vectors[comp], vectors[name])))
        m = MapVecPairToVecPair(stars[comp].vector,stars[name].vector, vectors[comp],vectors[name])
        print( "  Quat:", m.quat())
        if m.quat().rijk[0] > 0:
            quats.append(m.quat())
            pass
        else:
            quats.append(m.quat().neg())
            pass
        pass
    else:
        print( name, data[name], vectors[name])
        pass
    pass
print(quats)
avg = None
n = 0
for q in quats:
    if avg is None:
        avg = q
        n = 1
        pass
    else:
        c = dot_product(avg.axis(), q.axis())
        if c > 0.8:
            avg = avg.add(q)
            n += 1
            pass
        pass
    pass
avg = avg.normalized()
print("Average Q", avg)


for name in data:
    xyz = avg.apply_to(stars[name].vector)
    (x, y) = pxy_of_vec(xyz)
    dx = data[name][0] - x
    dy = data[name][1] - y
    err2 = dx*dx+dy*dy
    err = math.sqrt(err2)
    print(name, err, x, y, data[name])
    pass

xyz = avg.conj().apply_to(vec_of_pxy(2400, 1859))
best = (-2, None)
for name in stars:
    v = stars[name].vector
    d = dot_product(v,xyz)
    if d > best[0]:
        best = (d, id)
        pass
    pass
print(best, stars[best[1]])
# print(xyz, ra_de_of_v(xyz))



comp = "8 Draconis"
comp = "70 Ursa Majoris A"
comp = "70 Ursa Majoris A"
stars_checked = []
stars_to_check = [comp]
star_chain = []
chain_data = data.copy()
for ed in range(len(extra_data)):
    name = f"Extra {ed}"
    xy = extra_data[ed]
    chain_data[name] = xy
    vectors[name] = vec_of_pxy(xy[0], xy[1])
    pass
while len(stars_checked) < 15:
    print(stars_checked, stars_to_check)
    if stars_to_check == []: break
    comp = stars_to_check.pop()
    stars_checked.append(comp)
    r = []
    for name in chain_data:
        if name == comp: continue
        d = dot_product(vectors[comp], vectors[name])
        if d > Quadrant.cos_quad:
            r.append((name, d))
            if name not in stars_checked:
                stars_to_check.append(name)
                pass
            pass
        pass
    star_chain.append((comp,r))
    pass
print(star_chain)
def find_candidate(candidates, so_far, remaining_chain):
    """
    Given so_far as a dictionary
    """
    # print(f"Find {so_far} {remaining_chain}")
    if remaining_chain == []:
        candidates.append( so_far.copy())
        return
    (name,neighbours) = remaining_chain[0]
    next_remaining_chain = remaining_chain[1:]
    this_star = stars[so_far[name]]
    def find_neighbours_of_candidate(candidates,so_far, next_remaining_chain, neighbours_remaining):
        if neighbours_remaining == []:
            return find_candidate(candidates, so_far, next_remaining_chain)
        (nn, c) = neighbours_remaining[0]
        neighbours_remaining = neighbours_remaining[1:]
        ca = approx_angle_of_cos(c)
        for sn in this_star.neighbors:
            # 0.5 degrees is 9E-3
            # 0.5 degrees squared is 7E-5
            da = approx_angle_of_cos(sn[0]) - ca
            # if da*da < 7E-5:
            if da*da < 3E-6:
                if nn in so_far:
                    if so_far[nn] == sn[1]:
                        find_neighbours_of_candidate(candidates, so_far, next_remaining_chain, neighbours_remaining)
                        pass
                    pass
                else:
                    so_far[nn] = sn[1]
                    find_neighbours_of_candidate(candidates, so_far, next_remaining_chain, neighbours_remaining)
                    del(so_far[nn])
                    pass
                pass
            # else:
            #    if nn in so_far:
            #        return
            #    pass
            pass
        pass
    find_neighbours_of_candidate(candidates, so_far, next_remaining_chain, neighbours)
    pass
chain_start = star_chain[0][0]
candidates = []
for n in stars:
    find_candidate(candidates, {chain_start:n}, star_chain)
    pass
for c in candidates:
    print(c)
    pass

 */
