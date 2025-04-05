import React, { useEffect, useState } from 'react';
import axios from 'axios';
import { Line } from 'react-chartjs-2';
import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend } from 'chart.js';

ChartJS.register(CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend);

interface Repo {
  name: string;
  owner: string;
}

const App: React.FC = () => {
  const [repos, setRepos] = useState<Repo[]>([]);
  const [commitData, setCommitData] = useState<any>(null);

  useEffect(() => {
    axios.get('http://localhost:8080/api/repos')
      .then(response => setRepos(response.data));
  }, []);

  const fetchCommits = (owner: string, repo: string) => {
    axios.get(`http://localhost:8080/api/commits/${owner}/${repo}`)
      .then(response => {
        const data = {
          labels: response.data.map((w: any) => new Date(w.week * 1000).toLocaleDateString()),
          datasets: [{
            label: 'Commits',
            data: response.data.map((w: any) => w.total),
            borderColor: 'blue',
            fill: false,
          }],
        };
        setCommitData(data);
      });
  };

  return (
    <div>
      <h1>GitHub Dashboard</h1>
      <h2>Repositories</h2>
      <ul>
        {repos.map(repo => (
          <li key={repo.name}>
            {repo.name} <button onClick={() => fetchCommits(repo.owner, repo.name)}>Show Commits</button>
          </li>
        ))}
      </ul>
      {commitData && (
        <Line data={commitData} options={{ responsive: true, scales: { x: { title: { display: true, text: 'Week' } } } }} />
      )}
    </div>
  );
};

export default App;