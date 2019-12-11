package com.example.togglutopia.ui.log

import android.graphics.Color
import androidx.compose.Composable
import androidx.compose.unaryPlus
import androidx.ui.core.Text
import androidx.ui.core.dp
import androidx.ui.foundation.Clickable
import androidx.ui.foundation.VerticalScroller
import androidx.ui.layout.*
import androidx.ui.material.Button
import androidx.ui.material.MaterialTheme
import androidx.ui.material.ripple.Ripple
import androidx.ui.material.withOpacity
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.LogInteractions
import com.example.togglutopia.ui.Screen
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.navigateTo

@Composable
fun LogScreen(interactions: LogInteractions) {
    VerticalScroller {
        Padding(padding = 16.dp) {
            Column {
                val fullname = TogglState.user?.fullname ?: ""
                Text(text = "You are logged in as $fullname")
                Button(text = "Sync", onClick = { interactions.sync() })
                HeightSpacer(32.dp)
                TogglState.timeEntryList.forEach { te ->
                    TimeEntryRow(timeEntry = te)
                }
            }
        }

    }
}

@Composable
fun TimeEntryRow(timeEntry: TimeEntry) {
    Ripple(bounded = true) {
        Clickable(onClick = {
            navigateTo(Screen.Edit(timeEntry))
        }) {
            Row(modifier = Spacing(16.dp)) {
                Column(modifier = Flexible(1f)) {
                    Title(timeEntry)
                    Project(timeEntry)
                }
                TimeEntryDuration(timeEntry)
            }
        }
    }
}

@Composable
fun Title(timeEntry: TimeEntry) {
    Text("${timeEntry.id}: ${timeEntry.description}", style = ((+MaterialTheme.typography()).subtitle1).withOpacity(0.87f))
}

@Composable
fun Project(timeEntry: TimeEntry) {
    val project = TogglState.projectList.firstOrNull { it.id == timeEntry.project_id }
    if (project != null) {
        val style = ((+MaterialTheme.typography()).subtitle1).withOpacity(0.87f)
        Text(project.name, style = style)
    }
}

@Composable
fun TimeEntryDuration(timeEntry: TimeEntry) {
    Text("${timeEntry.duration}", style = ((+MaterialTheme.typography()).body2).withOpacity(0.87f))
}

@Preview
@Composable
fun preview() {
//    LogScreen()
}