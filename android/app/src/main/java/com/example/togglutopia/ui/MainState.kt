package com.example.togglutopia.ui

import androidx.compose.Model
import androidx.compose.frames.ModelList
import androidx.compose.frames.modelListOf
import com.example.togglutopia.data.model.Project
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.data.model.User

/**
 * Class defining the screens we have in the app: home, article details and interests
 */
sealed class Screen {
    data class Edit(val timeEntry: TimeEntry) : Screen()
    object Log : Screen()
}

@Model
object TogglState {
    var currentScreen: Screen = Screen.Log
    var user: User? = null
    var timeEntryList: ModelList<TimeEntry> = modelListOf()
    var projectList: ModelList<Project> = modelListOf()
}

/**
 * Temporary solution pending navigation support.
 */
fun navigateTo(destination: Screen) {
    TogglState.currentScreen = destination
}