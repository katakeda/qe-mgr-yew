use super::modal::{Modal, ModalProps};
use crate::common::ComponentProps;
use stylist::yew::styled_component;
use yew::{function_component, html, use_state, Callback};

#[styled_component(StyledCardEditModal)]
fn styled_card_edit_modal(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            display: flex;
            flex-direction: column;
            margin-top: 40px;
            width: 100%;
            padding: 16px;
            .card-detail-group {
                display: flex;
                justify-content: space-between;
                margin-top: 20px;
            }
            .card-detail-group > span {
                width: 20%;
            }
            .card-detail-label-group {
                display: flex;
                flex-grow: 1;
                margin-right: 5px;
            }
            .card-detail-title span {
                font-size: 22px;
            }
            .card-detail-title input {
                width: 100%;
            }
            .card-detail-desc textarea {
                margin-left: 3px;
                width: 100%;
                height: 65px;
                font-family: sans-serif;
                resize: none;
            }
            .card-detail-action-group {
                display: flex;
                justify-content: end;
                margin-top: 20px;
            }
            .card-detail-action-group > button {
                background-color: #f44336;
                color: #fff;
                border-radius: 3px;
                border: none;
                padding: 4px 6px 3px;
            }
            button {
                cursor: pointer;
            }
        ")}>{props.children.clone()}</div>
    }
}

#[function_component(CardEditModal)]
pub fn card_edit_modal(props: &ModalProps) -> Html {
    let editing_title = use_state(|| false);
    let editing_description = use_state(|| false);
    let editing_status = use_state(|| false);
    let editing_assigned_to = use_state(|| false);

    let refresh_ticket = {
        let editing_title = editing_title.clone();
        Callback::from(move |e| {
            editing_title.set(false);
        })
    };
    let save_ticket = {
        let editing_title = editing_title.clone();
        Callback::from(move |e| {
            editing_title.set(false);
        })
    };
    let delete_ticket = {
        let editing_title = editing_title.clone();
        Callback::from(move |_| {
            editing_title.set(false);
        })
    };

    html! {
        <Modal close={props.close.clone()}>
            <StyledCardEditModal>
                <div class="card-detail-group card-detail-title">
                    {if *editing_title {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    // <input type="text" bind:value={updatedTicket.title} />
                                    <input type="text" />
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={refresh_ticket}>{"Cancel"}</button>
                                    <button onclick={save_ticket}>{"Save"}</button
                                    >
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    // <span>{ticket.title}</span>
                                    <span>{"title"}</span>
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_title = editing_title.clone();
                                        Callback::from(move |_| {
                                            editing_title.set(true);
                                        })
                                    }}>{"Edit"}</button>
                                </div>
                            </>
                        }
                    }}
                </div>
                // <div class="card-detail-group card-detail-desc">
                //     <span>Description: </span>
                //     {#if editingDescription}
                //     <div class="card-detail-label-group">
                //         <textarea bind:value={updatedTicket.description} />
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button
                //         on:click={() => {
                //             editingDescription = false;
                //             resetTicket('description');
                //         }}>Cancel</button
                //         >
                //         <button
                //         on:click={() => {
                //             editingDescription = false;
                //             saveTicket({ description: updatedTicket.description });
                //         }}>Save</button
                //         >
                //     </div>
                //     {:else}
                //     <div class="card-detail-label-group">
                //         <span>{ticket.description}</span>
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button on:click={() => (editingDescription = true)}>Edit</button>
                //     </div>
                //     {/if}
                // </div>
                // <div class="card-detail-group">
                //     <span>Assigned To: </span>
                //     {#if editingAssignedTo}
                //     <div class="card-detail-label-group">
                //         <select bind:value={updatedTicket.assigned_to}>
                //         <option default selected value={{ id: '' }}>Unassigned</option>
                //         {#each availableUsers as user}
                //             <option
                //             value={user}
                //             selected={user.id == updatedTicket.assigned_to?.id}
                //             >{user.name}</option
                //             >
                //         {/each}
                //         </select>
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button
                //         on:click={() => {
                //             editingAssignedTo = false;
                //             resetTicket('assigned_to');
                //         }}>Cancel</button
                //         >
                //         <button
                //         on:click={() => {
                //             editingAssignedTo = false;
                //             saveTicket({ assigned_to: updatedTicket.assigned_to?.id });
                //         }}>Save</button
                //         >
                //     </div>
                //     {:else}
                //     <div class="card-detail-label-group">
                //         <span
                //         >{ticket.assigned_to ? ticket.assigned_to.name : 'Unassigned'}</span
                //         >
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button on:click={() => (editingAssignedTo = true)}>Edit</button>
                //     </div>
                //     {/if}
                // </div>
                // <div class="card-detail-group">
                //     <span>Status: </span>
                //     {#if editingStatus}
                //     <div class="card-detail-label-group">
                //         <select bind:value={updatedTicket.status}>
                //         {#each statuses as status}
                //             <option
                //             value={status.value}
                //             selected={status.value == updatedTicket.status}
                //             >{status.label}</option
                //             >
                //         {/each}
                //         </select>
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button
                //         on:click={() => {
                //             editingStatus = false;
                //             resetTicket('status');
                //         }}>Cancel</button
                //         >
                //         <button
                //         on:click={() => {
                //             editingStatus = false;
                //             saveTicket({ status: updatedTicket.status });
                //         }}>Save</button
                //         >
                //     </div>
                //     {:else}
                //     <div class="card-detail-label-group">
                //         <span>{statuses.find((s) => s.value === ticket.status)?.label}</span>
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button on:click={() => (editingStatus = true)}>Edit</button>
                //     </div>
                //     {/if}
                // </div>
                <div class="card-detail-action-group">
                    <button onclick={delete_ticket}>{"Delete"}</button>
                </div>
            </StyledCardEditModal>
        </Modal>
    }
}
