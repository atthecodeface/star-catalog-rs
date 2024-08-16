use std::error::Error;

use star_catalog::{hipparcos, Catalog};

#[cfg(test)]
const EXTRA_ALIASES: &[(usize, &'static str)] = &[
    (61281, "Kappa Draconis"),
    (56211, "Lambda Draconis"),
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

#[test]
fn test_read_hipparcos_json() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("hipparcos.json")?;
    let mut catalog: Catalog = serde_json::from_str(&s)?;
    catalog.sort();
    eprintln!("Loaded {} stars", catalog.len());
    catalog.add_names(hipparcos::HIP_ALIASES)?;
    catalog.add_names(EXTRA_ALIASES)?;
    Ok(())
}

/*
data = {
    "Dubhe": (3406, 3006),
    "Megrez": (1318, 2516),
    "Alioth": (451, 1771),
    "Phecda": (1265, 3448),
    "Kappa Draconis": (3096, 493),
    "Lambda Draconis": (3825, 1364),
    "3 Draconis": (3315, 1638),
    "4 Draconis": (3038, 616),
    "6 Draconis": (3123, 444),
    "8 Draconis": (2106, 655),
    "9 Draconis": (2260, 447),
    "7 Draconis": (2432, 649),
    "78 Ursa Majoris A": (420, 1555),
    "HD111456": (1303, 1380),
    "HD110678": (1527, 1398),
    "70 Ursa Majoris A": (1358, 2282),
    "74 Ursa Majoris A": (1278, 2017),
    "75 Ursa Majoris A": (1336, 1965),
    "HD105043": (2401, 1858),
}
extra_data = [
    (1813, 1236),
    (1746, 1117),
    (3375, 686),
#     (1300, 1376),
    (420,1556),
    ]

for name in data:
    if name not in stars:
        raise Exception(f"Bad data - star {name} not in stars database")
    pass

# What star is at 2400 1859?
cx = 5184/2
cy = 3456/2
mm_per_px = 22.3/5184
fl = 50.0 * 1.03

def pxy_of_vec(xyz):
    tx = xyz[0]
    ty = xyz[1]
    x = cx + (tx*fl / mm_per_px)
    y = cy - (ty*fl / mm_per_px)
    return (x,y)

def vec_of_pxy(x, y):
    # Get x and y as offsets in mm on frame
    x = (x - cx) * mm_per_px
    y = (cy - y) * mm_per_px
    # Lens is at focal length, so tan(angle) is x / f or y/f
    tx = x / fl
    ty = y / fl
    # Unit vector (from lens to frame, really) is then (tx, ty, 1) normalized assuming linear mapping for lens
    l2 = 1.0 + tx*tx + ty*ty
    l = math.sqrt(l2)
    v = (tx/l, ty/l, -1/l)
    return v

vectors = {}
print(22.3/5184, 14.9/3456)
for (name, (x,y)) in data.items():
    vectors[name] = vec_of_pxy(x,y)
    pass

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
