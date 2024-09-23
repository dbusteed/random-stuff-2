extends Node

func battle(a: Array, b: Array):	
	var atk: Array
	var def: Array
	
	if len(a) >= len(b):
		atk = a
		def = b
	else:
		atk = b
		def = a
	
	while len(atk) and len(def):
		atk.shuffle()
		def.shuffle()
		
		print('\n\nROUND')
			
		var battles = []
		for _i in range(len(def)):
			var bat = [def.pop_back(), atk.pop_back()]
			battles.append(bat)
		
		for i in range(len(battles)):
			if len(atk) == 0:
				break
			battles[i].append(atk.pop_back())
		
		var def1 = []
		var atk1 = []
		for bat in battles:
			var res = combat(bat)
			def1.append_array(res[0])
			atk1.append_array(res[1])
		
		atk.append_array(atk1)
		def.append_array(def1)

	print('\nDONE ', def, ' ', atk)

func combat(c):
	print('\nCOMBAT ', c)
	var b = c[0]
	var a = c.slice(1)
	var b_atk = b.attack()
	print('b_atk ', b_atk)
	
	var bonus = 0
	if len(a) > 1:
		bonus = 5
	
	var b_surv = true
	var a_surv = []
	for aa in a:
		var aa_atk = aa.attack() + bonus
		print('a_atk ', aa_atk)
		if aa_atk >= b_atk:
			a_surv.append(aa)
			if aa_atk > b_atk:
				b_surv = false
				
	if b_surv:
		return [[b], a_surv]
	else:
		return [[], a_surv]
		
	
