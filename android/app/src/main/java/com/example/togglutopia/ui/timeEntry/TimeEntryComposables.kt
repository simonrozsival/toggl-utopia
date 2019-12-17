package com.example.togglutopia.ui.timeEntry

import android.util.Log
import androidx.compose.Composable
import androidx.compose.State
import androidx.compose.unaryPlus
import androidx.ui.core.Text
import androidx.ui.core.TextField
import androidx.ui.core.dp
import androidx.ui.graphics.Color
import androidx.ui.layout.Column
import androidx.ui.layout.HeightSpacer
import androidx.ui.layout.Row
import androidx.ui.layout.Spacing
import androidx.ui.material.Button
import androidx.ui.material.MaterialTheme
import androidx.ui.material.OutlinedButtonStyle
import androidx.ui.material.withOpacity
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.LoginInteractions
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.login.LoginScreen
import com.example.togglutopia.utils.ISO8601

@Composable
fun TimeEntryContent(timeEntry: TimeEntry, isEditable: Boolean = false) {
    Row(modifier = Spacing(12.dp)) {
        Column(modifier = Flexible(1f)) {
            if (isEditable) EditableTitle(timeEntry) else Title(timeEntry)
            Project(timeEntry)
        }
        Column {
            TimeEntryDuration(timeEntry)
            HeightSpacer(height = 4.dp)
            if (timeEntry.duration == null) {
                Button(text = "Stop", onClick = {
                    TogglState.localEditTimeEntry(timeEntry.copy(duration = getRunningDuration(timeEntry) ,at = ISO8601.now(), edited = true))
                })
            } else {
                Button(text = "Start", onClick = {
                    TogglState.newRunningTimeEntry(timeEntry.description)
                }, style = OutlinedButtonStyle())
            }
        }
    }
}

@Composable
fun EditableTitle(timeEntry: TimeEntry) {
    TextField(
            value = timeEntry.description,
            onValueChange = { TogglState.localEditTimeEntry(timeEntry.copy(description = it, at = ISO8601.now(), edited = true)) },
            textStyle = ((+MaterialTheme.typography()).body2)
    )
}


@Composable
fun Title(timeEntry: TimeEntry) {
    var style = ((+MaterialTheme.typography()).subtitle1).withOpacity(0.87f)
    if (timeEntry.edited) {
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
    val d = timeEntry.duration ?: getRunningDuration(timeEntry)
    val duration = String.format("%d:%02d:%02d", d / 3600, (d % 3600) / 60, (d % 60));
    Text(duration, style = ((+MaterialTheme.typography()).body2).withOpacity(0.87f))
}

private fun getRunningDuration(timeEntry: TimeEntry): Int {
    val d = (TogglState.currentTime - ISO8601.toCalendar(timeEntry.start).time.time).toInt() / 1000
    return if (timeEntry.edited) d else d - 3600
}

@Preview(name = "Running")
@Composable
fun TimeEntryContent() {
    TimeEntryContent(TimeEntry(0, "", "Fist Time Entry", null, 1, "", "", 1 , true) , false)
}

@Preview(name = "Regular log")
@Composable
fun preview() {
    TimeEntryContent(TimeEntry(0, "", "Fist Time Entry", 130, 1, "", "", 1) , false)
}