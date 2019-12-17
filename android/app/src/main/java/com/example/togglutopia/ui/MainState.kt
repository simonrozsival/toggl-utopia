package com.example.togglutopia.ui

import androidx.compose.Model
import androidx.compose.State
import androidx.compose.frames.ModelList
import androidx.compose.frames.modelListOf
import androidx.compose.state
import androidx.compose.unaryPlus
import com.example.togglutopia.data.model.EntityUpdate
import com.example.togglutopia.data.model.Project
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.data.model.User
import com.example.togglutopia.utils.ISO8601
import java.util.*

@Model
object TogglState {
    var currentTime: Long = Date().time
    var currentScreen: Screen = Screen.MainLog
    var user: User? = null
    var timeEntryList: ModelList<TimeEntry> = modelListOf()
    var projectList: ModelList<Project> = modelListOf()

    fun localEditTimeEntry(timeEntry: TimeEntry) {
        timeEntryList.removeIf { it.id == timeEntry.id }
        timeEntryList.add(timeEntry)
    }

    fun editTimeEntry(entityUpdate: EntityUpdate<TimeEntry>) {
        val originalId = entityUpdate.client_assigned_id ?: entityUpdate.entity.id
        timeEntryList.removeIf { it.id == originalId }
        timeEntryList.add(entityUpdate.entity)
    }

    fun deleteTimeEntry(entityUpdate: EntityUpdate<TimeEntry>) {
        val originalId = entityUpdate.client_assigned_id ?: entityUpdate.entity.id
        timeEntryList.removeIf { it.id == originalId }
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
            timeEntryList.add(newTimeEntry)
        }
    }

    fun newRunningTimeEntry(description: String? = null) {
        user?.default_workspace_id?.let { workspaceId ->
            val lastNegativeId = timeEntryList.localOnly().minBy { it.id }?.id ?: -1
            val newTimeEntry = TimeEntry(
                    lastNegativeId - 1,
                    ISO8601.now(),
                    description ?: "New with running id: $lastNegativeId",
                    null,
                    null,
                    null,
                    ISO8601.now(),
                    workspaceId,
                    true
            )
            timeEntryList.add(newTimeEntry)
        }
    }
}

/**
 * Class defining the screens we have in the app: home, article details and interests
 */
sealed class Screen {
    data class EditTimeEntry(val timeEntryId: Int) : Screen()
    object MainLog : Screen()
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