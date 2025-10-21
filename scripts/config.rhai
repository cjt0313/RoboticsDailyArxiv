let titles_type = ["Dataset"];
let titles_model = ["manipulation", "locomotion", "Task Planning", "Motion Planning", "Long-term", "Planning", "Optimization", "quadruped", "legged robot", "Mobile Manipulation"];
let titles_method = ["physic", "task", "manipulation", "Decision", "Guassian", "differentiable", "simulation", "simulator", "dataset", "Deformable", 
"geometric", "active", "non prehensile", "grasp", "POMDP", "MDP", "trajectory", "GPT", "LLM", "embodied", "VLM", "VLA", "mobile"];
let titles =  titles_model + titles_method + titles_type;
let authors_array = [
    "Yuke Zhu", "Leslie Pack Kaelbling", "Joshua Tenenbaum", "Tomás Lozano-Pérez", "Panpan CAI", "David Hsu",
    "Wenjing Tang", "Xinyu He", "Yongxi Huang", "Yunxiao Xiao",
    "Wolfram Burgard", "Yunzhu Li", "Ken Goldberg", "Guanya Shi", "Lydia E Kavraki", "Zachary Kingston",
    "Marco Pavone", "Chelsea Finn", "Shuran Song", "Danfei Xu", "Siwei Chen", "Yiqing Xu", "Jiayuan Mao", "Yilun Du",
    "Marco Hutter", "Xiaolong Wang", "Li Fei-Fei", "Pieter Abbeel", "Abdeslam Boularias", "Michael Posa",
    "Shaojie Shen", "Tong Qin", "Hongyang Li", "Junchi Yan", "Karl Pertsch", "Boris Ivanovic", "Qihang Zhou",
    "Ken Goldenberg", "Deepak Pathek", "Sergey Levine", "Cewu Lu"];
    

let conferences = [
    "AAAI", "CORL", "RAS", "CoRL",
    "RA-L", "RAL", "IJRR", "ICRA", "IJCAI", "IROS",
    "UAI", "RSS", "TRO",
    "CVPR", "ECCV", "ICCV", "Transactions on Robotics",
    "International Journal of Robotics Research", "Robotics and Automation Letters", "ICLR", "ICML",
    "NeurIPS", "NIPS", "COLT"];

let start = conferences.shift();
let conferences_rg = conferences.reduce(|sum, v| sum + "|" + v, start);
let conferences_rg = "(" + conferences_rg + ")([\\s'-]*\\d+)?";

let start = titles.shift();
let title_rg = titles.reduce(|sum, v| sum + "|" + v, start);

let start = authors_array.shift();
let authors_rg = authors_array.reduce(|sum, v| sum + "|" + v, start);

export title_rg;
export authors_rg;
export conferences_rg;
