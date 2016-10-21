select * from fillup f
    join vehicle v on f.vehicle_id = v.id

where v.id = $1
order by f.id
offset $2
limit $3
