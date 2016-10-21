select * from vehicle v 
    join "user" u on v.user_id = u.id 

where u.id = $1
order by v.id
offset $2
limit $3
