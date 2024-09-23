import json
import xmltodict

x = open('dev/mini_golf.drawio.xml', 'r').read()
x = xmltodict.parse(x)

# game window
w = 900 / 2
h = 700 / 2

out = []

levels = x['mxfile']['diagram']
for i in range(len(levels)):
    obj = {}
    
    obj['par'] = int(levels[i]['@name'].split(' - ')[1])
    nodes = levels[i]['mxGraphModel']['root']['mxCell']
    
    for n in nodes:
        if '@value' not in n:
            continue
        
        # 1. there's only one of each of these, so handle them separately
        # 2. we want the circle origin to be in the center, so offset the (x, y)
        #    by (width / 2, height / 2) to convert top-left to center origin
        # 3. we also adjust the coordinates with half the game window size
        #    to transform the draw.io coordinates (which start top left, and has 
        #    x go left-to-right and y goes top-to-bottom) to Bevy coordinates, which
        #    is centered at 0, and uses a standard (x, y) plane
        if n['@value'] in ('ball', 'hole'):
            obj[n['@value']] = {
                'x': (int(n['mxGeometry']['@x']) + (int(n['mxGeometry']['@width']) / 2)) - w, 
                'y': h - (int(n['mxGeometry']['@y']) + (int(n['mxGeometry']['@height']) / 2)),
            }

        elif n['@value'] in ('chaser'):
            if n['@value'] not in obj:
                obj[n['@value']] = []
            obj[n['@value']].append({
                'x': (int(n['mxGeometry']['@x']) + (int(n['mxGeometry']['@width']) / 2)) - w, 
                'y': h - (int(n['mxGeometry']['@y']) + (int(n['mxGeometry']['@height']) / 2)),
            })            

        # 1. we can have multiple walls, greens, etc, so put these in a list
        # 2. we do similar offset as we did with the ball/hole
        elif n['@value'] in ('green', 'wall'):
            if n['@value'] not in obj:
                obj[n['@value']] = []
            obj[n['@value']].append({
                'x': (int(n['mxGeometry']['@x']) + (int(n['mxGeometry']['@width']) / 2)) - w,
                'y': h - (int(n['mxGeometry']['@y']) + (int(n['mxGeometry']['@height']) / 2)),
                'width': float(n['mxGeometry']['@width']),
                'height': float(n['mxGeometry']['@height']),
            })

        if 'chaser' not in obj:
            obj['chaser'] = []
        
        if 'patrol' not in obj:
            obj['patrol'] = []

    out.append(obj)

with open('assets/levels.json', 'w') as f:
    f.write(json.dumps(out, indent=2))

# # sort the dict keys so everything is in same order
# out = list(map(lambda d: dict(sorted(d.items())), out))

# f = open('src/level_data.rs', 'w')
# f.write("""pub struct Position {
#     pub x: f32,
#     pub y: f32,
# }

# pub struct Rectangle {
#     pub x: f32,
#     pub y: f32,
#     pub width: f32,
#     pub height: f32,
# }


# pub struct LevelData {
#     pub ball: Position,
#     pub chaser: Vec<Position>,
#     pub green: Vec<Rectangle>,
#     pub hole: Position,
#     pub par: usize,
#     pub patrol: Vec<Vec<Position>>,
#     pub wall: Vec<Rectangle>,
# }

# pub const LEVELS: Vec<LevelData> = vec![
# """)

# for level in out:
#     f.write("\tLevelData {\n")
#     for ent, data in level.items():
#         if ent == 'par':
#             f.write(f"\t\t{ent}: {data},\n")
#         elif ent == 'chaser':
#             f.write(f"\t\t{ent}: vec![\n")
#             for e in data:
#                 f.write(f"\t\t\tPosition {{ x: {e['x']}, y: {e['y']} }},\n")
#             f.write(f"\t\t],\n")                
#         elif ent in ('ball', 'hole'):
#             f.write(f"\t\t{ent}: Position {{ x: {data['x']}, y: {data['y']} }},\n")
#         elif ent in ('green', 'wall'):
#             f.write(f"\t\t{ent}: vec![\n")
#             for e in data:
#                 f.write(f"\t\t\tRectangle {{")
#                 f.write(f" x: {e['x']}, y: {e['y']}, width: {e['width']}, height: {e['height']}")
#                 f.write(f" }},\n")
#             f.write(f"\t\t],\n")
#     f.write('\t\tpatrol: vec![],\n')
#     f.write("\t},\n")
# f.write('];')

# f.close()
