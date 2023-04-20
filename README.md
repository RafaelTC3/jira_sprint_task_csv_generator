# jira_sprint_task_csv_generator
Rust application to generate csv file with multiple subtasks to be uploaded in Jira

This application receives a template file to generate a csv file to be uploaded into Jira.

The motivation beihind the creation of this application is to help those who need to implement multiple subtsks, but the option is blocked for the team.

As already mentioned, the template file will looke like this:

```
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
```
# Docker Image and Container

## First mode to use the container
### Step 1 
Enter in project folder and enter command:
```
docker build -t <name you want to call the image> .
```

After creating the image enter the following command:
```
docker run -it --name <container name> -d <image name:version>
```

Now you have a running container.
### Step 2
Copy the wanted file inside the container using docker cp command:
```
docker cp <file path in your local machine> <container name>:<path in the container>
```

### Step 3
Execute the container in interactive mode with command:
```
docker exec -it <container name> bash
```

### Step 4
Execute the binary passing the arguments:
```
<path to binary> <file path> <sprint id> <squad>
```

### Step 5
Copy the wanted file from the container using docker cp command:
```
docker cp <container name>:<path in the container> <path in your local machine>
```

### Step 6
Exit container wrinting exit in terminal.

## Second mode to use the container


### Incomming updates
* Docker image to run in container
* Pass args to generate template to be used
* Configuration to upload to cloud (Google Drive, Azure etc...)

