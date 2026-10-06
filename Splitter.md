
AI Generated Game made in Bevy (0.19.1)

NOTE THIS IS WITH BEVY 0.19.1 VERIFIY ALL APIs ARE VALID.

# Challenge Rules
- Cards - The game must have card element
- Minimal enemy count = 200 at 60FPS
- 2 Boss's and 4 types of enemies
- At least 2 types of damage types (not Elemental Damage) - Unique ways damage is APPLIED. 
-  No art, must be shapes
- Must have pause and start menu
- Must have "Dave" - "D.A.V.E" - (IE some elemnt or character named dave)
- Dont make it 1 file - must be an actual project
- No AI agents, only chatbot
- No manual additions or implementation - limiting scope of manual hand edit 

So MY game is going to be a 2d Wave survivor (vampire survivors like thingy).

# Highlights

Phyiscs based weapons and enemy "breakage". 
Enemies are not just health bars (they will still have one) but are composed over various "limbs" that can be destroyed. So a main body (still a limb) and legs and such. Remember its still all just 2d shapes.
Limbs can have armor, protecting it from projectile weapons.

High enemy count (Since bevy ECS can do really well, should have zero problems hitting 200 at 60 fps, should do closer to 20 k at that)

Two main forms of weapons, Projectile and Laser. 

# Weapons
## Projectile weapons

Damage based on mass(which is also render size) and speed.

Some projectiles can pierce loosing speed at each pierece. 

Parts that are destroyted by project weapons gain the speed that the projectile looses, becoming a new projectile, which will also damage other enemies. 

No damage fall off over range (Speed is only lost on hitting enemies and limbs)

Blocked by armor unless it can pen.

Projects will have travel time. And will have impact based collisions.

AOE projectile weapons wil fire a projectile that explodes into more that each deal damage. (AOE projectiles will unqiuely have a max range)

Will have to reload between shots, or wait until the loader from the magazine. 
Then after the magazine is dry, you will need to reload. 

For example a cannon will have to load each round after firing, but a machine gun would have a magazine. 



## Laser weapons

DOT based damage with an Intensity stat. (High intensity can mimic a hard hitting laser)
Intensity fall off over range based on focus stat.

Lasers will have infinite pierce but will loose damage over range. 
They will be hitscan.

Lasers will NOT trigger limbs going flying, but instead will cause them to detonate dealing raw AOE damage in a small area. 

After firing a laser weapon will have two things, a recharge and a cooling cycle.
A laser weapon cannot be fired before the recharge is done, but it CAN be fired before the weapon is fully cooled off, and if it does. It willl turn of shield regen. 

## Unlocks


The player will unlock new weapons and upgrades via cards, once per wave. 

The player will be able to hold two weapons. One projectile and one laser. 

They can dual wield a weapon in ONE slot (so two projectile and one laser, or vis versa) but dual wielded weapons have 50% reduced projectile mass. 



# Enemies

Enemies will NOT have any form of damage or hp scaling. Static for the entire game, the only way they scale is quantity. But they will be fairly strong by default. 


Enemies will be made of "smooth" shapes, circles, elipses, cylinders. any shape with minimal edges. They will be made of more vibrrant colors like reds, greens , and yellows. 

## Attacks

Enemies will have both ranges attacks and contact damage. Simple but reliable


## Bosses

Bosses will be larger enemies with special effects. IE healing AOE circles (that can regrow limbs) and other various systems.

### Dave requirement

D.A.V.E will be a Copy of the player, and won't need to have a crazy complicated AI, it SHOULD know how to fire the weapons, and aim the main turret to a basic degree. 

## Defense

Enemies will have two main defense stats, (alongside hp) hardness and bulk. 

Bulky will be countered by the mass stat of a projectile weapon. if your mass stat is lower than the bulk stat, you will read reduced damage.  Bulk will also reduce laser damage.

Hardness will be countered by the speed stat of a projectile weapon. If you speed stat is lower than the hardness stat, you will deal reduced damage. Hardness WILL NOT affect laser damage. 

## Limbs

Enemies will be composed of multiple "limbs" a main body limb, and a collection of motion limbs, and or a ranged attack limb.

Motion Limbs being destroyed will reduce enemy movement speed. While destroying a ranged attack limb will stop that enemy from attacking with its range attack. 

Limbs will not have collision with other enemies (only the main body will be checked for that, for performance reasons) BUT limbs will be checked for on collision of weapons. 

When a limb is destroyed it will have the effect based on the weapon described in the weapons section. 

Limbs can be "over piereced" by laser weapons and piecering projectile weapons. 

Over piereced limbs will take damage like normal, except the main body. If a limb was hitm BEFORE the main body. The main body because immune to that attack, UNLESS the limb is destroyed. 

## Armor

Limbs can have armor pieces which will have its own unqiue hardness and bulk stats. 

Armor will NOT have the same weapon effects that limbs have when destroyted. They just protect limbs, and will NOT be sent flying.  

A limb cannot be damaged if armor exists anywhere on it. It is NOT based on collisions.

armor protects against an extra "pierece" distance for projectile weapons, and will REFLECT laser weapons. (SO it will reflect in another direction) I think this will add for some fun chaos to the laser weapons.  Lasers weapons will have a hidden reflection cap (mostly for performance) 


# Player

A small tank, that is trying to fend of hordes of bug like creatures. 

the tank can have A single turret, (left click which follows mouse). and a front mounted weapon, right click weapon which cannot do anything but fire forward. The weapon "type" that will go where is chosen BEFORE the run. So a laser or projectile can be on the turret, but that is chosen before the run starts. Not in the run.

The coaxial weapon will have a 100% bonus to projectile mass. 

Again, the player can choose to dual weild (in run) a certain weapon, a player has a hard cap of 3 weapons (so only 1 dual wield ,and 1 single) Again like I mentioned it will cause a 50% reductiion in projectile mass (and conversly size).

Players will be made of shapes with "rough" edges like sqaures and rectangles, trapazoids, and triangles. Along with White, Gray and blue. 
## Looses
IF the player takes to much damage, they will loose.


## Movement

The player cannot strafe, only forward/backwards and steer, its a tank. 

the player will have a short "blink" (its an advanced tank), which can ONLY go side to side. (The blink is a strafing blink, but you can't strafe normally)


WASD is for moving and turning. Q and E will strafe blink in the respetive directions.

The main turret follows the mouse and is fired with left click, the coaxial weapon is fired with right click, and only shoots straight.

## Shield

The player will have a weak but vital shield, there is NO way to heal health. The shield regenerates over time. ANY pure health damage taken is PERMANANT.

## Upgrades

The classic, movement speed, shield upgrades. There will be no health upgrades. Only shield.


Upgrades to weapons will cover a huge amount of things. 

Things like (extra barrels) for the turret, or (larger magazine). For projectile weapons, (since all projectile weapons will be mag fed or single shot). Single shot weapons cannot get larger mags. They can get extra barrels.

Things like larger capactiors and better cooling for the laser weapons, since they will be cooldown based in two factors. Temp, and Energy. 

## Camera

Fixed, battle arena and camera, but it will be FAIRLY large in relation to the player tank. (think will look quite small). 


## World space

THe world will be a orangeish color, background. For now just leave it one color. 


# Project Structure

- `main.rs` (App initialization, Game State definitions, and Plugin registration)
    
- `player/` (Tank movement, blink logic, input handling)
    
- `weapons/` (Projectile logic, laser hitscan, firing cooldowns)
    
- `enemies/` (Limb mechanics, D.A.V.E. spawning, baseline tracking)
    
- `physics/` (Impact collisions, mass vs. bulk calculations, limb detachment)
    
- `ui/` (Main menu, pause menu, card selection)
    
- `waves/` (Wave progression, enemy spawning)
    
- `core/` (Shared components and utilities)