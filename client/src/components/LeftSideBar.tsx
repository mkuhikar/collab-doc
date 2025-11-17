 export default function LeftSideBar(){
    return (
<div className="col-12 col-md-3 col-lg-2 bg-white shadow-sm p-4 sidebar-custom">


                        <ul className="list-unstyled sidebar-menu">
                            <li className="active">
                                <i className="bi bi-journal-text me-2"></i> All Notes
                            </li>
                            <li >
                                <i className="bi bi-star me-2"></i> Favorites
                            </li>
                            <li >
                                <i className="bi bi-tags me-2"></i> Tags
                            </li>
                            <li >
                                <i className="bi bi-people me-2"></i> Shared with me
                            </li>
                            <li >
                                <i className="bi bi-clock-history me-2"></i> Recent Notes
                            </li>
                            <li >
                                <i className="bi bi-trash me-2"></i> Trash
                            </li>
                        </ul>

                        <hr />

                        <p className="fw-semibold small text-muted">Tags</p>
                        <div className="d-flex flex-column gap-1">
                            <span><span className="tag-dot bg-warning"></span> Work</span>
                            <span><span className="tag-dot bg-primary"></span> Ideas</span>
                            <span><span className="tag-dot bg-success"></span> Personal</span>
                            <span><span className="tag-dot bg-info"></span> Projects</span>
                        </div>

                        
                    </div>
    )
 }
 
 