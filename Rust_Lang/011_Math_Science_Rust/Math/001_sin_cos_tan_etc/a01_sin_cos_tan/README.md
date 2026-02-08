# Result

```bash
=== Triangulation for Distance Measurement ===

--- Example 1: Basic Triangulation ---
Scenario: Two surveyors are 100m apart, measuring angles to a landmark
Baseline: 100m
Angle from point A: 45°
Angle from point B: 60°
Distance from A: 89.66m
Estimated position: (63.40, 63.40)
Confidence: 83.3%

--- Example 2: Building Height Measurement ---
Scenario: Measure building height from a known distance
Distance to building: 50m
Elevation angle: 30°
Building height: 28.87m

--- Example 3: Elevation Triangulation ---
Scenario: Measure tower height from two positions
Distance moved: 30m
Angle (near): 50°
Angle (far): 35°
Tower height: 50.93m
Distance from near position: 42.74m

--- Example 4: 3D Triangulation ---
Scenario: Track a flying drone
Baseline: 50m
3D Distance: 41.39m
Position: x=33.68m, y(z)=14.16m (height)

--- Example 5: Stellar Parallax ---
Scenario: Calculate distance to a nearby star
Earth's orbital diameter: 2.99e11 m
Stellar parallax: 0.76 arcseconds
Distance to star: 40601.60e12 m (4.291 light-years)

--- Trigonometric Reference ---
sin(30°) = 0.5000
cos(60°) = 0.5000
tan(45°) = 1.0000

Note: sin(θ) = cos(90°-θ)

```

# AI
- I've created a comprehensive Rust triangulation example with **5 different techniques** using sin, cos, and tan:

| Example | Use Case | Key Formula |
|---------|----------|-------------|
| **1. Basic Triangulation** | Surveying landmark from two points | Law of sines: `d × sin(β) / sin(α + β)` |
| **2. Building Height** | Height from known distance | `distance × tan(elevation_angle)` |
| **3. Elevation Triangulation** | Tower height from two positions | `(d × tan(α) × tan(β)) / (tan(β) - tan(α))` |
| **4. 3D Triangulation** | Track flying drone | Combines azimuth + elevation |
| **5. Stellar Parallax** | Distance to stars | Uses Earth's orbit as baseline |

**Key Functions:**
- `basic_triangulation()` - Two-point triangulation using sin
- `right_triangle_height()` - Simple tan-based height calculation
- `elevation_triangulation()` - Height when distance is unknown
- `triangulation_3d()` - Full 3D positioning
- `parallax_distance()` - Astronomical distance measurement

All results are now correct with positive values. The code is well-documented with ASCII diagrams showing the geometric relationships.
