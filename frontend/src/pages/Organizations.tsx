import React, { useState, useEffect } from "react";
import {
  Box,
  Typography,
  Card,
  CardContent,
  Grid,
  CircularProgress,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  Collapse,
} from "@mui/material";
import {
  Business as BusinessIcon,
  Code as CodeIcon,
  ExpandLess,
  ExpandMore,
} from "@mui/icons-material";
import { apiService } from "../services/api";

interface Repository {
  owner: string;
  name: string;
}

interface Organization {
  id: number;
  login: string;
  description: string | null;
  avatar_url: string;
  html_url: string;
  repos_url: string;
  type: string;
}

const Organizations: React.FC = () => {
  const [organizations, setOrganizations] = useState<Organization[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [expandedOrgs, setExpandedOrgs] = useState<Set<string>>(new Set());

  useEffect(() => {
    const fetchOrganizations = async () => {
      try {
        setLoading(true);
        const response = await apiService.getOrganizations();
        setOrganizations(response.data as Organization[]);
        setError(null);
      } catch (err: any) {
        console.error("Error fetching organizations:", err);
        setError(err.message || "Failed to load organizations");
      } finally {
        setLoading(false);
      }
    };

    fetchOrganizations();
  }, []);

  const handleToggleOrg = (orgName: string) => {
    setExpandedOrgs((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(orgName)) {
        newSet.delete(orgName);
      } else {
        newSet.add(orgName);
      }
      return newSet;
    });
  };

  if (loading) {
    return (
      <Box
        display="flex"
        justifyContent="center"
        alignItems="center"
        minHeight="200px"
      >
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box p={3}>
        <Typography color="error">{error}</Typography>
      </Box>
    );
  }

  return (
    <Box p={3}>
      <Typography variant="h4" gutterBottom>
        Your Organizations
      </Typography>

      <Grid container spacing={3}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <List>
                {organizations.map((org) => (
                  <React.Fragment key={org.name}>
                    <ListItem button onClick={() => handleToggleOrg(org.name)}>
                      <ListItemIcon>
                        <BusinessIcon />
                      </ListItemIcon>
                      <ListItemText
                        primary={org.name}
                        secondary={`${org.repositories.length} repositories`}
                      />
                      {expandedOrgs.has(org.name) ? (
                        <ExpandLess />
                      ) : (
                        <ExpandMore />
                      )}
                    </ListItem>
                    <Collapse
                      in={expandedOrgs.has(org.name)}
                      timeout="auto"
                      unmountOnExit
                    >
                      <List component="div" disablePadding>
                        {org.repositories.map((repo) => (
                          <ListItem
                            key={`${repo.owner}/${repo.name}`}
                            button
                            component="a"
                            href={`https://github.com/${repo.owner}/${repo.name}`}
                            target="_blank"
                            rel="noopener noreferrer"
                            sx={{ pl: 4 }}
                          >
                            <ListItemIcon>
                              <CodeIcon />
                            </ListItemIcon>
                            <ListItemText primary={repo.name} />
                          </ListItem>
                        ))}
                      </List>
                    </Collapse>
                  </React.Fragment>
                ))}
              </List>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
};

export default Organizations;
