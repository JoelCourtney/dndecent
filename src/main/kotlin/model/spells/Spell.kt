package model.spells

import model.quantities.*

data class Spell(
    val name: String,

    val level: Int,
    val school: SpellSchool,

    val castingTime: Time,
    val ritual: Boolean,

    val range: Distance,

    val verbal: Boolean,
    val somatic: Boolean,
    val material: Boolean,

    val duration: Time,
    val concentration: Boolean,

    val description: String,

    val displayInCombat: Boolean,
    val displayInRoleplay: Boolean

) {
    val availableTo: Array<String> = arrayOf()
    val materialDescription: String = ""
}

