# jira_sprint_task_csv_generator
Rust application to generate csv file with multiple subtasks to be uploaded in Jira

This application receives a template file to generate a csv file to be uploaded into Jira.

The motivation beihind the creation of this application is to help those who need to implement multiple subtsks, but the option is blocked for the team.

As already mentioned, the template file will looke like this:

//* Represents History 
//-- Represents subtasks
//| Represents the estimated hours to resolve the task
//< Represents the type of the Task
//Do not use double quotes " inside the file
*13000
--Task Summary | 10 < Sub-Imp
	+ comment for the task
	+ Can have multiple comments
	+ comment with example on ho to write 'specific information' without double quotes


