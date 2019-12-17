package com.example.togglutopia.ui.edit

import androidx.compose.Composable
import androidx.compose.state
import androidx.compose.unaryPlus
import androidx.ui.core.dp
import androidx.ui.layout.Column
import androidx.ui.layout.Spacing
import androidx.ui.material.RadioGroup
import androidx.ui.material.surface.Card
import androidx.ui.tooling.preview.Preview
import com.example.togglutopia.data.model.TimeEntry
import com.example.togglutopia.ui.TogglState
import com.example.togglutopia.ui.timeEntry.TimeEntryContent

@Composable
fun EditScreen(timeEntryId: Int) {
    val editedTimeEntry = TogglState.timeEntryList.find { it.id == timeEntryId } ?: return

    Column(modifier = Spacing(16.dp)){
        TimeEntryContent(editedTimeEntry, true)
        if (!TogglState.projectList.isEmpty()) {
            ProjectSelector(editedTimeEntry)
        }
    }

}

@Composable
fun ProjectSelector(timeEntry: TimeEntry) {
    val projectOptions = TogglState.projectList.filter { it.active }.map { it.name }
    val selectedProject = TogglState.projectList.find { it.id == timeEntry.project_id }
    val (selectedOption, _) = +state { selectedProject?.name }
    RadioGroup(
            options = projectOptions,
            selectedOption = selectedOption,
            onSelectedChange = { projectName ->
                val project = TogglState.projectList.first { it.name == projectName }
                TogglState.localEditTimeEntry(timeEntry.copy(project_id = project.id))
            }
    )
}

@Preview
@Composable
fun preview() {
    TogglState.timeEntryList.addAll(listOf(
            TimeEntry(1, "", "Fist Time Entry", 130, 1, "", "", 1)
    ))
    EditScreen(1)
}



