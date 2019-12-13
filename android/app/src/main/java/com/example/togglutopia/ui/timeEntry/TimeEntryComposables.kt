package com.example.togglutopia.ui.timeEntry

import androidx.compose.Composable
import androidx.compose.unaryPlus
import androidx.ui.core.Text
import androidx.ui.core.TextField
import androidx.ui.core.dp
import androidx.ui.graphics.Color
import androidx.ui.layout.Column
import androidx.ui.layout.Row
import androidx.ui.layout.Spacing
import androidx.ui.material.MaterialTheme
import androidx.ui.material.withOpacity

import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.utils.ISO8601

@Composable
fun TimeEntryContent(timeEntry: TimeEntry, isEditable: Boolean = false) {
    Row(modifier = Spacing(16.dp)) {
        Column(modifier = Flexible(1f)) {
            if (isEditable) EditableTitle(timeEntry) else Title(timeEntry)
            Project(timeEntry)
        }
        TimeEntryDuration(timeEntry)
    }
}

@Composable
fun EditableTitle(timeEntry: TimeEntry) {
    TextField(
            value = timeEntry.description,
            onValueChange = { TogglState.editTimeEntry(timeEntry.copy(description = it, at = ISO8601.now(), edited = true)) },
            textStyle = ((+MaterialTheme.typography()).body2)
    )
}


@Composable
fun Title(timeEntry: TimeEntry) {
    var style = ((+MaterialTheme.typography()).subtitle1).withOpacity(0.87f)
    if (timeEntry.id < 0) {
        style = style.copy(color = Color.Red)
    }
    Text(timeEntry.description, style = style)
}

@Composable
fun Project(timeEntry: TimeEntry) {
    val project = TogglState.projectList.firstOrNull { it.id == timeEntry.project_id }
    if (project != null) {
        var style = ((+MaterialTheme.typography()).subtitle1).withOpacity(0.87f)
        val parsedColor = android.graphics.Color.parseColor(project.color)
        style = style.copy(color = Color(parsedColor))
        Text(project.name, style = style)
    }
}

@Composable
fun TimeEntryDuration(timeEntry: TimeEntry) {
    val duration = String.format("%d:%02d:%02d", timeEntry.duration / 3600, (timeEntry.duration % 3600) / 60, (timeEntry.duration % 60));
    Text(duration, style = ((+MaterialTheme.typography()).body2).withOpacity(0.87f))
}