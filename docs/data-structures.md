# Chapter 5: Data Structures

[← Functions & Modules](functions-and-modules.md) • [Wiki Home](README.md) • [Next: Error Handling →](error-handling.md)

---

## 1. Dynamic Arrays

Arrays in Alya are dynamically sized heap buffers. They support 0-based indexing, fast mutation, and automatic bounds checking.

```alya
let items = [10, 20, 30]
say items.len()     # 3
say items[0]        # 10

# Dynamic growth
items.push(40)
say items           # [10, 20, 30, 40]

# Pop last element
let last = items.pop()
say last            # 40

# In-place element mutation
items[1] = 99
items[0] += 5
say items           # [15, 99, 30]
```

### Unsigned Bounds Check Optimization
Alya verifies bounds using a single unsigned assembly comparison (`jae` / `b.hs`). Any negative index (`< 0`) wraps into an astronomically large unsigned number exceeding array length, catching underflows and overflows simultaneously with zero branch misprediction penalty.

---

## 2. Hash Maps (`map()`)

Hash maps provide associative key-value storage indexed by strings:

```alya
let user = map()
user["name"] = "Alice"
user["role"] = "Admin"
user.set("level", 10)

say user["name"]            # Alice
say user.get("role")        # Admin
say user.contains("level")  # 1
say user.len()              # 3

# Keys and iteration
for key in user.keys()
    say "{key} -> {user[key]}"
end

# Removal
user.remove("role")
say user.has("role")        # 0
```

---

## 3. Structs & Custom Types

Structs allow bundling heterogeneous fields into structured composite records:

```alya
struct Vector2D
    x
    y
end

# Named constructor
let v1 = Vector2D { x: 3.0, y: 4.0 }

# Positional constructor
let v2 = Vector2D(1.0, 2.0)

# Field access and mutation
v1.x += 10.0
say "v1 coordinates: ({v1.x}, {v1.y})"
```

### Compile-Time Struct Type Inference
Alya runs an interprocedural multi-pass analysis (fixed-point traversal) that statically resolves struct types across function calls, return expressions, and local variables:

```alya
function create_vector(x, y)
    return Vector2D(x, y)
end

let v = create_vector(10.0, 20.0)
v.x += 5.0     # The compiler infers 'v' is Vector2D and computes field 'x' offset statically!
```

This guarantees zero dynamic dictionary lookups and zero boxing overhead for field access.

---

## 4. Progressive Examples

### Level 1: Pure & Minimal (Basic Array & Map)
```alya
# Array operations
let list = []
list.push(100)
list.push(200)
say list.len()    # 2
say list[0]       # 100

# Map operations
let scores = map()
scores["Alice"] = 95
scores["Bob"] = 80
say scores["Alice"]    # 95
```

---

### Level 2: Practical & Idiomatic (Student Grade Management)
Combining structs and collections to calculate GPA rankings:

```alya
struct Student
    name
    score
end

function get_letter_grade(score)
    if score >= 90
        return "A"
    elif score >= 80
        return "B"
    elif score >= 70
        return "C"
    else
        return "F"
    end
end

let roster = [
    Student { name: "Alice", score: 94 },
    Student { name: "Bob",   score: 83 },
    Student { name: "Clara", score: 72 },
    Student { name: "Dan",   score: 61 }
]

let total_points = 0

say "=== Student Grade Report ==="
for s in roster
    let grade = get_letter_grade(s.score)
    total_points += s.score
    say "{s.name}: {s.score} pts (Grade: {grade})"
end

let class_average = float(total_points) / float(roster.len())
say "----------------------------"
say "Class Average: {class_average} pts"
```

---

### Level 3: Advanced & Real-World (2D Physics Particle Simulation)
A multi-entity particle simulation updating velocities, handling boundary bounces, and computing kinetic energy:

```alya
struct Particle
    id
    x
    y
    vx
    vy
    mass
end

function create_particle(id, x, y, vx, vy, mass)
    return Particle {
        id: id,
        x: float(x),
        y: float(y),
        vx: float(vx),
        vy: float(vy),
        mass: float(mass)
    }
end

function update_particle(p, dt, box_w, box_h)
    p.x += p.vx * dt
    p.y += p.vy * dt

    # Bounce off horizontal walls
    if p.x <= 0.0
        p.x = 0.0
        p.vx = 0.0 - p.vx
    elif p.x >= box_w
        p.x = box_w
        p.vx = 0.0 - p.vx
    end

    # Bounce off vertical walls
    if p.y <= 0.0
        p.y = 0.0
        p.vy = 0.0 - p.vy
    elif p.y >= box_h
        p.y = box_h
        p.vy = 0.0 - p.vy
    end
end

function kinetic_energy(p)
    let speed_sq = p.vx * p.vx + p.vy * p.vy
    return 0.5 * p.mass * speed_sq
end

# Initialize particle system
let particles = [
    create_particle(1, 10, 20,  5,  3, 2),
    create_particle(2, 50, 80, -4,  6, 1),
    create_particle(3, 90, 40,  2, -8, 3)
]

let box_width = 100.0
let box_height = 100.0
let delta_time = 0.5
let steps = 3

for step in 1..steps
    say "--- Simulation Step #{step} ---"
    let total_energy = 0.0

    for p in particles
        update_particle(p, delta_time, box_width, box_height)
        let ke = kinetic_energy(p)
        total_energy += ke
        say "Particle #{p.id}: pos=({p.x}, {p.y}), vel=({p.vx}, {p.vy}), KE={ke}"
    end

    say "System Kinetic Energy: {total_energy}"
end
```
