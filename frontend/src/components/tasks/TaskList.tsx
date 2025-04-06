import React, { useState, useEffect } from "react";
import {
    Box,
    Typography,
    LinearProgress,
    Card,
    CardContent,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    Paper,
    IconButton,
    Chip,
} from "@mui/material";
import { Edit as EditIcon, Delete as DeleteIcon } from "@mui/icons-material";
import { taskService, Task } from "../../services/taskService";

interface TaskListProps {
    filters?: {
        status?: string;
        priority?: string;
        repo?: string;
    };
}

const TaskList: React.FC<TaskListProps> = ({ filters }) => {
    const [tasks, setTasks] = useState<Task[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchTasks = async () => {
            try {
                setLoading(true);
                const response = await taskService.listTasks(filters);
                setTasks(response.data.tasks);
                setError(null);
            } catch (error) {
                console.error("Error fetching tasks:", error);
                setError("Failed to load tasks");
            } finally {
                setLoading(false);
            }
        };

        fetchTasks();
    }, [filters]);

    const handleDelete = async (id: number) => {
        try {
            await taskService.deleteTask(id);
            setTasks(tasks.filter((task) => task.id !== id));
        } catch (error) {
            console.error("Error deleting task:", error);
        }
    };

    if (loading) {
        return <LinearProgress />;
    }

    if (error) {
        return <Typography color="error">{error}</Typography>;
    }

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Tasks
            </Typography>

            <TableContainer component={Paper}>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell>Title</TableCell>
                            <TableCell>Priority</TableCell>
                            <TableCell>Status</TableCell>
                            <TableCell>Due Date</TableCell>
                            <TableCell>Actions</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {tasks.map((task) => (
                            <TableRow key={task.id}>
                                <TableCell>{task.title}</TableCell>
                                <TableCell>
                                    <Chip
                                        label={task.priority}
                                        color={
                                            task.priority === "high"
                                                ? "error"
                                                : task.priority === "medium"
                                                    ? "warning"
                                                    : "success"
                                        }
                                    />
                                </TableCell>
                                <TableCell>
                                    <Chip
                                        label={task.status}
                                        color={
                                            task.status === "open"
                                                ? "error"
                                                : task.status === "in-progress"
                                                    ? "warning"
                                                    : "success"
                                        }
                                    />
                                </TableCell>
                                <TableCell>{new Date(task.due_date).toLocaleDateString()}</TableCell>
                                <TableCell>
                                    <IconButton
                                        color="primary"
                                        onClick={() => {
                                            // Handle edit
                                        }}
                                    >
                                        <EditIcon />
                                    </IconButton>
                                    <IconButton
                                        color="error"
                                        onClick={() => handleDelete(task.id)}
                                    >
                                        <DeleteIcon />
                                    </IconButton>
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </TableContainer>
        </Box>
    );
};

export default TaskList; 