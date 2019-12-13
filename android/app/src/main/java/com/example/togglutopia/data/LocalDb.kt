package com.example.togglutopia.data

import android.content.Context
import androidx.compose.frames.ModelList
import androidx.core.content.edit
import com.example.togglutopia.data.model.Project
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.data.model.User
import com.example.togglutopia.ui.TogglApp
import com.example.togglutopia.ui.TogglState
import com.squareup.moshi.JsonAdapter
import com.squareup.moshi.Moshi
import com.squareup.moshi.Types


class LocalDb(context: Context) {

    private val sharedPreferences = context.getSharedPreferences(PREF_KEY, Context.MODE_PRIVATE)
    private val moshi = Moshi.Builder().build()
    private val userAdapter = moshi.adapter(User::class.java)
    private val timeEntriesAdapter = getListAdapter(TimeEntry::class.java)
    private val projectAdapter = getListAdapter(Project::class.java)

    fun restoreState() {
        sharedPreferences.getString(USER_KEY, null)?.apply {
            TogglState.user = userAdapter.fromJson(this)
        }
        sharedPreferences.getString(TIME_ENTRIES_KEY, null)?.apply {
            TogglState.timeEntryList.clear()
            TogglState.timeEntryList.addAll(timeEntriesAdapter.fromJson(this)!!)
        }
        sharedPreferences.getString(PROJECTS_KEY, null)?.apply {
            TogglState.projectList.clear()
            TogglState.projectList.addAll(projectAdapter.fromJson(this)!!)
        }
    }

    fun persistState() {
        sharedPreferences.edit(commit = true) {
            putString(USER_KEY, userAdapter.toJson(TogglState.user))
            putString(TIME_ENTRIES_KEY, timeEntriesAdapter.toJson(TogglState.timeEntryList.toList()))
            putString(PROJECTS_KEY, projectAdapter.toJson(TogglState.projectList.toList()))
        }
    }

    var lastSync: String?
        get() = sharedPreferences.getString(LAST_SYNC_KEY, null)
        set(value) {
            sharedPreferences.edit(commit = true) {
                putString(LAST_SYNC_KEY, value)
            }
        }

    private fun <T> getListAdapter(clazz: Class<T>): JsonAdapter<List<T>> {
        val type = Types.newParameterizedType(List::class.java, clazz)
        return moshi.adapter<List<T>>(type)
    }

    companion object {
        const val PREF_KEY = "utopia"
        const val USER_KEY = "user"
        const val TIME_ENTRIES_KEY = "time_entries"
        const val PROJECTS_KEY = "projects"
        const val LAST_SYNC_KEY = "last_sync"
    }
}