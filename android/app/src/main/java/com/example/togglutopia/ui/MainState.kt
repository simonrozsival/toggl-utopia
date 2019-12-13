package com.example.togglutopia.ui

import androidx.compose.Model
import androidx.compose.State
import androidx.compose.frames.ModelList
import androidx.compose.frames.modelListOf
import androidx.compose.state
import androidx.compose.unaryPlus
import com.example.togglutopia.data.model.Project
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.data.model.User
import com.example.togglutopia.utils.ISO8601
import java.util.*

@Model
object TogglState {
    var currentTime: Long = Date().time
    var currentScreen: Screen = Screen.Log
    var user: User? = null
    var timeEntryList: ModelList<TimeEntry> = modelListOf()
    var projectList: ModelList<Project> = modelListOf()

    fun addTimeEntry(timeEntry: TimeEntry) {
        timeEntryList.add(timeEntry)
    }

    fun editTimeEntry(timeEntry: TimeEntry) {
        timeEntryList.removeIf { it.id == timeEntry.id }
        timeEntryList.add(timeEntry)
    }

    fun deleteTimeEntry(timeEntry: TimeEntry) {
        timeEntryList.removeIf { it.id == timeEntry.id }
    }

    fun clearLocalTimeEntries() {
        timeEntryList.removeIf { it.id < 0 }
    }

    fun newTimeEntry() {
        user?.default_workspace_id?.let { workspaceId ->
            val lastNegativeId = timeEntryList.localOnly().minBy { it.id }?.id ?: -1
            val newTimeEntry = TimeEntry(
                    lastNegativeId - 1,
                    ISO8601.now(),
                    "New with id: $lastNegativeId",
                    20,
                    null,
                    null,
                    ISO8601.now(),
                    workspaceId,
                    true
            )
            addTimeEntry(newTimeEntry)
        }
    }

    fun newRunningTimeEntry() {
        user?.default_workspace_id?.let { workspaceId ->
            val lastNegativeId = timeEntryList.localOnly().minBy { it.id }?.id ?: -1
            val newTimeEntry = TimeEntry(
                    lastNegativeId - 1,
                    ISO8601.now(),
                    "New with running id: $lastNegativeId",
                    null,
                    null,
                    null,
                    ISO8601.now(),
                    workspaceId,
                    true
            )
            addTimeEntry(newTimeEntry)
        }
    }
}

/**
 * Class defining the screens we have in the app: home, article details and interests
 */
sealed class Screen {
    data class Edit(val timeEntryId: Int) : Screen()
    object Log : Screen()
}

fun Iterable<TimeEntry>.localOnly(): List<TimeEntry> {
    return this.filter { it.id < 0 }
}

/**
 * Temporary solution pending navigation support.
 */
fun navigateTo(destination: Screen) {
    TogglState.currentScreen = destination
}