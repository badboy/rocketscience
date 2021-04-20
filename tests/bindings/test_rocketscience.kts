import org.mozilla.uniffi.example.arithmetic.*;
import net.rediger.uniffi.rocketscience.*

val command = Part(name = "Mk1 Command Pod", cost = 600, weight = 840)
val tank = Part(name = "FL-T200", cost = 275, weight = 1125)
val engine = Part(name = "LV-T30", cost = 1100, weight = 1250)

val rocket = Rocket(name = "Orbiter")
rocket.add(part = command)
rocket.add(part = tank)
rocket.add(part = engine)

rocket.lockSteering(direction = Direction.DOWN)

try {
    rocket.launch()
    throw RuntimeException("Should have thrown a LaunchError exception!")
} catch(e: LaunchError) {
    // It's okay!
}

rocket.lockSteering(direction = Direction.UP)
assert(rocket.launch())
