use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::{
    api::ticket::TicketFilter,
    model::{team::Team, ticket::Ticket, user::User},
};

#[derive(Serialize, Deserialize)]
pub struct Data {
    users: HashMap<String, User>,
    tickets: HashMap<String, Ticket>,
    teams: HashMap<String, Team>,
}

pub struct Inmem {
    file_name: String,
    users: Arc<Mutex<HashMap<String, User>>>,
    tickets: Arc<Mutex<HashMap<String, Ticket>>>,
    teams: Arc<Mutex<HashMap<String, Team>>>,
}

impl Drop for Inmem {
    fn drop(&mut self) {
        let data = Data {
            users: self.users.clone().lock().unwrap().clone(),
            tickets: self.tickets.clone().lock().unwrap().clone(),
            teams: self.teams.clone().lock().unwrap().clone(),
        };
        fs::write(&self.file_name, serde_json::ser::to_string(&data).unwrap()).unwrap();
    }
}

impl Inmem {
    pub fn new(file_name: String) -> Self {
        let last_state = fs::read(&file_name).ok();
        let new_state = Inmem {
            file_name,
            users: Arc::new(Mutex::new(HashMap::new())),
            tickets: Arc::new(Mutex::new(HashMap::new())),
            teams: Arc::new(Mutex::new(HashMap::new())),
        };
        match last_state {
            Some(content) => {
                let data = serde_json::de::from_slice::<Data>(&content)
                    .expect("failed to parse data file");
                for user in data.users.iter() {
                    new_state
                        .users
                        .lock()
                        .unwrap()
                        .insert(user.0.clone(), user.1.clone());
                }
                for ticket in data.tickets.iter() {
                    new_state
                        .tickets
                        .lock()
                        .unwrap()
                        .insert(ticket.0.clone(), ticket.1.clone());
                }
                for team in data.teams.iter() {
                    new_state
                        .teams
                        .lock()
                        .unwrap()
                        .insert(team.0.clone(), team.1.clone());
                }
            }
            None => (),
        }
        new_state
    }

    pub fn get_users(&self) -> Vec<User> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, _user)| true)
            .map(|(_, user)| user.clone())
            .collect()
    }

    pub fn get_user(&self, id: String) -> Option<User> {
        match self.users.lock().unwrap().get(&id) {
            Some(user) => Some(user.clone()),
            None => None,
        }
    }

    pub fn create_user(&self, user: User) {
        self.users.lock().unwrap().insert(user.id.clone(), user);
    }

    pub fn _delete_user(&self, id: String) {
        self.users.lock().unwrap().remove(&id);
    }

    pub fn get_teams(&self) -> Vec<Team> {
        self.teams
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, _team)| true)
            .map(|(_, team)| team.clone())
            .collect()
    }

    pub fn get_team(&self, id: String) -> Option<Team> {
        match self.teams.lock().unwrap().get(&id) {
            Some(team) => Some(team.clone()),
            None => None,
        }
    }

    pub fn create_team(&self, team: Team) {
        self.teams.lock().unwrap().insert(team.id.clone(), team);
    }

    pub fn get_tickets(&self, filter: &TicketFilter) -> Vec<Ticket> {
        self.tickets
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, ticket)| {
                let mut filtered = true;
                if let Some(status) = &filter.status {
                    filtered &= *status == ticket.status;
                }
                if let Some(team) = &filter.team {
                    if !team.trim().is_empty() {
                        match self.get_team(ticket.team_id.clone()) {
                            Some(t) => filtered &= *team == t.name,
                            None => filtered &= false,
                        }
                    }
                }
                filtered
            })
            .map(|(_, ticket)| ticket.clone())
            .collect()
    }

    pub fn get_ticket(&self, id: String) -> Option<Ticket> {
        match self.tickets.lock().unwrap().get(&id) {
            Some(ticket) => Some(ticket.clone()),
            None => None,
        }
    }

    pub fn create_ticket(&self, ticket: Ticket) {
        self.tickets
            .lock()
            .unwrap()
            .insert(ticket.id.clone(), ticket);
    }

    pub fn delete_ticket(&self, id: String) {
        self.tickets.lock().unwrap().remove(&id);
    }
}
