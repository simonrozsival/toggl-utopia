package com.example.togglutopia.ui.edit

import androidx.compose.Composable
import androidx.ui.core.Text
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.TogglApp
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.timeEntry.TimeEntryContent

@Composable
fun EditScreen(timeEntryId: Int) {
    val editedTimeEntry = TogglState.timeEntryList.find { it.id == timeEntryId }
    editedTimeEntry?.apply {
        TimeEntryContent(this, true)
    }
}

