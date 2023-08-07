import rocketscience
import Foundation

let command = Part(name: "Mk1 Command Pod", cost: 600, weight: 840)
let tank = Part(name: "FL-T200", cost: 275, weight: 1125)
let engine = Part(name: "LV-T30", cost: 1100, weight: 1250)

let rocket = Rocket(name: "Orbiter")
rocket.add(part: command)
rocket.add(part: tank)
rocket.add(part: engine)

rocket.lockSteering(direction: .down)

do {
    let _ = try rocket.launch()
    fatalError("Should have thrown a LaunchError exception!")
} catch LaunchError.RocketLaunch {
    // It's okay!
}

rocket.lockSteering(direction: .up)
assert(try! rocket.launch())

var group = DispatchGroup()
group.enter()
let task = Task {
    rocket.lockSteering(direction: .up)
    // We can launch the rocket asynchronously
    let result = try await launchAfter(rocket: rocket, ms: 20)
    assert(result)

    group.leave()
}
group.wait(wallTimeout: .now() + .seconds(1))
